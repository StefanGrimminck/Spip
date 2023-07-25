# Spip - Internet Sensor

### Introduction:
Spip is an internet sensor designed in Rust. It uses the asynchronous features of Tokio and integrates networking functions from C via libc. Spip's primary role is to log all incoming TCP traffic. This tool is suitable for integration into network setups or on devices that are exposed to the internet, like a VPS.

### Use Cases:
By monitoring TCP traffic, Spip helps in identifying unusual network patterns or scanning attempts. It creates detailed records of all inbound TCP events, which are useful for threat analysis. These records are in JSON format.

If you have devices exposed to the internet, deploying multiple instances of Spip can enhance network monitoring.

### Design Philosophy:
Spip is designed to be efficient. It focuses on logging traffic without additional features like reverse DNS lookup or geolocation. If you need such features, you can add them during the data processing phase.
Output Details:

Spip provides the following data in JSON format:

    Timestamp
    Payload and Hex version
    Source and Destination IPs
    Source and Destination Ports
    Session ID

Example Output:
```json{
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


## Setting Up Spip:

1. Ensure you have Docker installed.

2. Go to the project's root directory.

3. Make the setup script executable:
```bash
chmod +x scripts/setup_spip_agent.sh
```

4. Run the setup script:

```bash
sudo ./scripts/setup_spip_agent.sh
```
This will compile Spip and create a config.toml in the `spip-output` directory.

5. Direct all TCP traffic to the port Spip will monitor. Exclude the port used by your SSH server:
```bash
sudo iptables -t nat -A PREROUTING -p tcp --dport <YOUR_SSH_SERVER_PORT> -j ACCEPT
sudo iptables -t nat -A PREROUTING -p tcp -j REDIRECT --to-port 12345

```

6. Start Spip:

```
./spip-output/spip-agent
```

## Advanced Integrations

- [Integrate spip-agent with Elasticsearch](./docs/ElasticsearchIntegration.md)


## Future Enhancements:
- Adding TLS support.
- Including UDP traffic monitoring.

## Contributing:
Contributions to Spip are welcome. Feel free to fork, make changes, and submit a pull request. For any issues, you can raise them on our GitHub repository.

## Contact:

For questions or feedback, please email me at spip@stefangrimminck.nl.

