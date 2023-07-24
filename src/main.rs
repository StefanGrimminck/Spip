use std::fs;
use std::io::Error;
use std::net::IpAddr;
use std::os::unix::io::AsRawFd;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use libc::{getsockopt, socklen_t, sockaddr_in, SOL_IP, SO_ORIGINAL_DST};
use native_tls;
use openssl::pkcs12::Pkcs12;
use openssl::pkey::PKey;
use openssl::rsa::Rsa;
use openssl::x509::X509;
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
    key_path: String,
    cert_path: String,
}

#[derive(Debug)]
struct OriginalDst {
    ip: IpAddr,
    port: u16,
}

#[derive(Clone)]
struct ServerConfig {
    ip: String,
    port: u16,
    identity: native_tls::Identity,
}

#[derive(Serialize)]
struct ConnectionData {
    timestamp: String,
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

async fn handle_client(mut stream: TcpStream, server_config: ServerConfig) {
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
                            timestamp: Utc::now().to_string(),
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

    // Read key and certificate files
    let key = fs::read(&config.key_path)
        .expect("Failed to read key file");
    let cert = fs::read(&config.cert_path)
        .expect("Failed to read certificate file");

    // Create key and cert objects
    let key = PKey::private_key_from_pem(&key).expect("Failed to create key");
    let cert = X509::from_pem(&cert).expect("Failed to create certificate");

    // Combine the key and cert into a PKCS #12 archive
    let pkcs12 = Pkcs12::builder()
        .build("", "", &key, &cert)
        .expect("Failed to build PKCS #12 archive");

    // Convert PKCS #12 archive into native_tls::Identity
    let identity = native_tls::Identity::from_pkcs12(&pkcs12.to_der().unwrap(), "")
        .expect("Failed to create identity");

    let server_config = ServerConfig {
        ip: config.ip,
        port: config.port,
        identity,
    };

    let bind_address = format!("{}:{}", server_config.ip, server_config.port);

    match TcpListener::bind(&bind_address).await {
        Ok(listener) => {
            loop {
                if let Ok((stream, _)) = listener.accept().await {
                    tokio::spawn(async move {
                        handle_client(stream, server_config.clone()).await;
                    });
                }
            }
        },
        Err(_) => eprintln!("Failed to bind to the address {}", &bind_address),
    }
}
