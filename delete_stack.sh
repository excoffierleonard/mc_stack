#!/bin/bash

# Check if the server ID is provided
if [ -z "$1" ]; then
    echo "Usage: $0 <minecraft_server_id>"
    exit 1
fi

server_id=$1
server_dir="minecraft_server_$server_id"

# Check if the directory exists
if [ ! -d "$server_dir" ]; then
    echo "Directory $server_dir does not exist."
    exit 1
fi

# Navigate to the server directory
cd "$server_dir"

# Stop and remove the Docker containers
docker compose down

# Navigate back to the parent directory
cd ..

# Remove the server directory
rm -rf "$server_dir"

echo "Minecraft server $server_id has been deleted."