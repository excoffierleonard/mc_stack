#!/bin/bash

# Set the base directory to the script's directory
BASE_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
STACKS_DIR="$BASE_DIR/stacks"

# Function to fetch the base port values from the original .env file
fetch_base_ports() {
  base_server_port=$(grep '^SERVER_PORT=' "$BASE_DIR/template/.env" | cut -d '=' -f 2)
  base_rcon_port=$(grep '^RCON_PORT=' "$BASE_DIR/template/.env" | cut -d '=' -f 2)
  base_sftp_port=$(grep '^SFTP_SERVER_PORT=' "$BASE_DIR/template/.env" | cut -d '=' -f 2)
}

# Find the highest existing stack_NUMBER directory
highest_number=$(ls -d "$STACKS_DIR/stack_"* 2>/dev/null | grep -o '[0-9]*' | sort -n | tail -1)
if [ -z "$highest_number" ]; then
  highest_number=0
fi

# Increment the number
new_stack_id=$((highest_number + 1))
new_stack_dir="$STACKS_DIR/stack_$new_stack_id"
new_stack_compose_file="$new_stack_dir/compose.yaml"

# Create the new directory
mkdir -p "$new_stack_dir"

# Copy compose.yaml and .env to the new directory
cp "$BASE_DIR/template/compose.yaml" "$BASE_DIR/template/.env" "$new_stack_dir"

# Fetch the base port values from the original .env file
fetch_base_ports

# Calculate new ports
increment=3
new_server_port=$((base_server_port + new_stack_id * increment))
new_rcon_port=$((base_rcon_port + new_stack_id * increment))
new_sftp_port=$((base_sftp_port + new_stack_id * increment))

# Update the .env file in the new directory
sed -i "s/^MINECRAFT_SERVER_SERVICE=.*/MINECRAFT_SERVER_SERVICE=minecraft_server_$new_stack_id/" "$new_stack_dir/.env"
sed -i "s/^MINECRAFT_SERVER_VOLUME=.*/MINECRAFT_SERVER_VOLUME=minecraft_server_$new_stack_id/" "$new_stack_dir/.env"
sed -i "s/^MINECRAFT_SERVER_NETWORK=.*/MINECRAFT_SERVER_NETWORK=minecraft_server_$new_stack_id/" "$new_stack_dir/.env"
sed -i "s/^SERVER_PORT=.*/SERVER_PORT=$new_server_port/" "$new_stack_dir/.env"
sed -i "s/^RCON_PORT=.*/RCON_PORT=$new_rcon_port/" "$new_stack_dir/.env"
sed -i "s/^SFTP_SERVER_PORT=.*/SFTP_SERVER_PORT=$new_sftp_port/" "$new_stack_dir/.env"
sed -i "s/^SFTP_SERVER_SERVICE=.*/SFTP_SERVER_SERVICE=sftp_server_$new_stack_id/" "$new_stack_dir/.env"

# Echo all the new static values
echo "New directory: $new_stack_dir"
echo "New SERVER_PORT: $new_server_port"
echo "New RCON_PORT: $new_rcon_port"
echo "New SFTP_SERVER_PORT: $new_sftp_port"
echo "New MINECRAFT_SERVER_SERVICE: minecraft_server_$new_stack_id"
echo "New MINECRAFT_SERVER_VOLUME: minecraft_server_$new_stack_id"
echo "New MINECRAFT_SERVER_NETWORK: minecraft_server_$new_stack_id"
echo "New SFTP_SERVER_SERVICE: sftp_server_$new_stack_id"

# Start the Docker containers using docker compose -f
docker compose -f "$new_stack_compose_file" up -d

# Echo the success message
echo "Stack $new_stack_id has been successfully created."

#TODO: Maybe add a memory to the last number used because edge case where the last highest is deleted and the next one is created with the same number
