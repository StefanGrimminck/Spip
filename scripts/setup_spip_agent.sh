#!/bin/bash

# Build the Docker image
docker build -t spip-compiler -f Dockerfile .

# Check if Docker build was successful
if [ $? -ne 0 ]; then
    echo "Failed to build the Docker image."
    exit 1
fi

# Run a temporary container
docker run --name temp-container spip-compiler /bin/true

# Copy the spip-agent executable from the container to the current directory
docker cp temp-container:/usr/src/spip/target/release/spip-agent .

# Remove the temporary container
docker rm temp-container

# If the copy command fails (e.g., because the path is incorrect), exit the script
if [[ ! -f spip-agent ]]; then
    echo "Error: Could not find the spip-agent binary."
    exit 1
fi

generate_config() {
    IP_ADDRESS=$(hostname -I | awk '{print $1}')
    cat > spip-output/config.toml <<EOL
ip = "$IP_ADDRESS"
port = 12345
EOL
}

# Create the output directory and move the spip-agent there
mkdir -p spip-output
mv spip-agent spip-output/

# Check if executable was moved correctly
if [[ ! -f spip-output/spip-agent ]]; then
    echo "Error: Failed to move spip-agent to spip-output/"
    exit 1
fi

# Generate config.toml and place it in the spip-output directory
generate_config

# Check if config was created correctly
if [[ ! -f spip-output/config.toml ]]; then
    echo "Error: Failed to generate config.toml in spip-output/"
    exit 1
fi

echo "config.toml generated successfully with IP: $IP_ADDRESS"

