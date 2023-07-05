use std::fs;
use std::io::Error;
use std::net::IpAddr;
use std::os::unix::io::AsRawFd;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use libc::{getsockopt, socklen_t, sockaddr_in, SOL_IP, SO_ORIGINAL_DST};
use serde_derive::{Deserialize, Serialize};
use serde_json;
use tokio::io::{AsyncReadExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::timeout;
use toml;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct Config {
    ip: String,
    port: u16,
}

#[derive(Debug)]
struct OriginalDst {
    ip: IpAddr,
    port: u16,
}

#[derive(Serialize)]
struct ConnectionData {
    timestamp: u64,
    payload: String,
    payload_hex: String,
    source_ip: String,
    source_port: u16,
    destination_ip: String,
    destination_port: u16,
    session_id: String,
}

fn read_config() -> Result<Config, Error> {
    let contents = fs::read_to_string("config.toml")?;
    toml::from_str(&contents).map_err(|err| Error::new(std::io::ErrorKind::InvalidData, err))
}

unsafe fn get_original_dst(socket: &TcpStream) -> Result<OriginalDst, Error> {
    let mut addr: sockaddr_in = std::mem::zeroed();
    let mut addr_len = std::mem::size_of::<sockaddr_in>() as socklen_t;

    let res = getsockopt(
        socket.as_raw_fd(),
        SOL_IP,
        SO_ORIGINAL_DST,
        &mut addr as *mut _ as *mut _,
        &mut addr_len,
    );

    if res == 0 {
        let port = u16::from_be(addr.sin_port);
        let ip = IpAddr::V4(std::net::Ipv4Addr::from(u32::from_be(addr.sin_addr.s_addr)));

        Ok(OriginalDst { ip, port })
    } else {
        Err(Error::last_os_error())
    }
}

async fn handle_client(mut stream: TcpStream) {
    let session_id = Uuid::new_v4().to_string();
    let mut buf = [0; 4096];

    loop {
        match timeout(Duration::from_secs(10), stream.read(&mut buf)).await {
            Ok(Ok(size)) => {
                if size == 0 {
                    break; // Client closed the connection, exit the loop
                }

                let payload = String::from_utf8_lossy(&buf[..size]).into_owned();
                let payload_hex = hex::encode(&buf[..size]);

                // Get original destination
                if let Ok(original_dst) = unsafe { get_original_dst(&stream) } {
                    if let Ok(peer_addr) = stream.peer_addr() {
                        let connection_data = ConnectionData {
                            timestamp: SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .unwrap()
                                .as_secs(),
                            payload,
                            payload_hex,
                            source_ip: peer_addr.ip().to_string(),
                            source_port: peer_addr.port(),
                            destination_ip: original_dst.ip.to_string(),
                            destination_port: original_dst.port,
                            session_id: session_id.clone(), // Use the same session ID for each message
                        };

                        if let Ok(json) = serde_json::to_string(&connection_data) {
                            println!("{}", json);
                        }
                    }
                }
            }
            Ok(Err(_)) | Err(_) => {
                break; // Exit the loop on read error or timeout error
            }
        }
    }
}


#[tokio::main]
async fn main() {
    let config = match read_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Failed to read configuration: {}", e);
            return;
        }
    };

    let bind_address = format!("{}:{}", config.ip, config.port);

    match TcpListener::bind(&bind_address).await {
        Ok(listener) => {
            loop {
                if let Ok((stream, _)) = listener.accept().await {
                    tokio::spawn(async move {
                        handle_client(stream).await;
                    });
                }
            }
        },
        Err(_) => eprintln!("Failed to bind to the address {}", &bind_address),
    }
}
