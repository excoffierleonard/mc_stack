#!/bin/bash

base_dir="$(dirname "$(realpath "$0")")/../stacks"

# Check if the stacks directory exists
if [ ! -d "$base_dir" ]; then
    echo "Error: Stacks directory $base_dir does not exist."
    exit 1
fi

# List all stack directories and sort them by stack ID
stack_dirs=$(find "$base_dir" -maxdepth 1 -type d -name 'stack_*' | sort -t'_' -k2,2n)

if [ -z "$stack_dirs" ]; then
    echo "No stacks found."
    exit 0
fi

# Get all running container names
running_containers=$(docker ps --format '{{.Names}}')

# Get WAN address
wan_address=$(wget -qO- http://ipinfo.io/ip)

# Iterate over each stack directory and check its status
for stack_dir in $stack_dirs; do
    stack_id=$(basename "$stack_dir" | cut -d'_' -f2)
    stack_compose_file="$stack_dir/compose.yaml"

    if [ ! -f "$stack_compose_file" ]; then
        echo "Stack $stack_id: compose.yaml not found."
        continue
    fi

    # Check if the SFTP server container is running
    sftp_status="stopped"
    for container_name in $running_containers; do
        if [[ $container_name == *"sftp_server_${stack_id}"* ]]; then
            sftp_status="running"
            break
        fi
    done

    # Check if the Minecraft server container is running
    minecraft_status="stopped"
    for container_name in $running_containers; do
        if [[ $container_name == *"minecraft_server_${stack_id}"* ]]; then
            minecraft_status="running"
            break
        fi
    done

    # Assuming default ports for SFTP and Minecraft servers
    sftp_port=22
    minecraft_port=25565

    echo "Stack $stack_id: SFTP server is $sftp_status (Address: $wan_address:$sftp_port), Minecraft server is $minecraft_status (Address: $wan_address:$minecraft_port)"
done