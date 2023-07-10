# Spip - Robust Internet Sensor

Spip is a minimalist internet sensor, built in Rust to leverage the asynchronous runtime of Tokio, and utilizing low-level networking capability from C via libc. It's tailored for integration within network infrastructure or deployment on an internet-facing device, such as a VPS. Its primary function is to log all incoming TCP traffic.

Spip can be used to identify abnormal network traffic patterns or attempts by internet scanners to locate specific devices. It generates a record of all inbound TCP network traffic, providing data in JSON format for threat detection and analysis.

When deployed in an internet-facing environment, multiple instances of Spip can be used for a broader range of network traffic surveillance and pattern detection of network activity.

Maintaining its lean design principle, Spip concentrates on recording traffic, avoiding tasks like reverse DNS lookup and geolocation. These features can be incorporated in the post-processing stage as needed, ensuring Spip remains resource-efficient.

## Output

Spip will output the following information in JSON format to Stdout:
- Timestamp
- Payload
- Hex payload
- Source IP
- Source Port
- Destination IP
- Destination Port
- Session ID

Example of a payload (addresses censored):

```JSON
{
  "timestamp": 1688737798,
  "payload": "u0013BitTorrent protocol",
  "payload_hex": "13426974546f7272656e742070726f746f636f6c",
  "source_ip": "146.70.x.x",
  "source_port": 35882,
  "destination_ip": "146.190.x.x",
  "destination_port": 6881,
  "session_id": "bd30cdc1-95b0-49aa-b8fe-e77230b6a04f"
}
```

## Configuration

Spip uses a configuration file named `config.toml` located in the same directory as the executable. The file should have the following format:

```toml
ip = "x.x.x.x" # IP Address for Spip to bind to.
port = 12345 # Port for Spip to bind to. Iptable commands should reflect this port. 
```

You need to specify the IP address and port number in the configuration file.

## Environment Setup

For the successful operation of Spip, it's essential to redirect TCP traffic from all ports to the port Spip will bind to. Remember to exclude the 
port where the SSH server binds to as well. Execute the following commands in your terminal to set up the environment:

``` bash
sudo iptables -t nat -A PREROUTING -p tcp --dport <YOUR_SSH_SERVER_PORT> -j ACCEPT
sudo iptables -t nat -A PREROUTING -p tcp -j REDIRECT --to-port <SPIP_PORT>
```

## Compilation Instructions

To compile the code using Cargo, run the following command in the terminal:

``` bash
cargo build --release
```

This command will compile your program with optimizations and generate an executable in the `target/release/` directory.

## Start-up Instructions

Make sure the `config.toml` file is present in the same directory as the executable. Then, run your instance using the following command:

```bash
./spip-agent
```

Cargo will handle the compilation and execution of your program.

## To-Do

- TLS support: setup handshake when TLS Client Hello is received
- UDP support

## Contributions

We invite and appreciate contributions to the project. You're welcome to fork the project and submit a pull request. If you encounter any issues, 
feel free to raise them in the GitHub repository.

## Contact Information

For any queries, suggestions, or feedback, please reach out to us at [spip@stefangrimminck.nl](mailto:spip@stefangrimminck.nl). We'll be delighted 
to assist you.
```
