#!/bin/bash

# Check if the server ID is provided
if [ -z "$1" ]; then
    echo "Usage: $0 <minecraft_server_id>"
    exit 1
fi

stack_id=$1
stack_dir="minecraft_server_$stack_id"
stack_compose_file="$stack_dir/compose.yaml"

# Check if the directory exists
if [ ! -d "$stack_dir" ]; then
    echo "Error: Directory $stack_dir does not exist."
    exit 1
fi

# Check if the docker-compose.yml file exists
if [ ! -f "$stack_compose_file" ]; then
    echo "Error: $stack_compose_file does not exist."
    exit 1
fi

# Bring down the Docker containers
docker compose -f "$stack_compose_file" down

# Remove the server directory
rm -rf "$stack_dir"

echo "Minecraft server $stack_id has been successfully deleted."