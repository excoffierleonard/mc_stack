#!/bin/bash

base_dir="$(dirname "$(realpath "$0")")/../stacks"
wan_ip=$(wget -qO- http://ipinfo.io/ip)

[ ! -d "$base_dir" ] && echo "Error: Stacks directory $base_dir does not exist." && exit 1

# List all stack directories and sort them by stack ID
stack_dirs=$(find "$base_dir" -maxdepth 1 -type d -name 'stack_*' | sort -t'_' -k2,2n)
[ -z "$stack_dirs" ] && echo "No stacks found." && exit 0

# Get all running containers with their ports
container_info=$(docker ps --format '{{.Names}}|{{.Ports}}')

# Iterate over each stack directory and check its status
for stack_dir in $stack_dirs; do
    stack_id=$(basename "$stack_dir" | cut -d'_' -f2)
    [ ! -f "$stack_dir/compose.yaml" ] && echo "Stack $stack_id: compose.yaml not found." && continue

    # Initialize variables
    sftp_status="stopped"
    sftp_port=""
    minecraft_status="stopped"
    minecraft_port=""

    while IFS='|' read -r name ports; do
        if [[ $name == *"sftp_server_${stack_id}"* ]]; then
            sftp_status="running"
            sftp_port=$(echo "$ports" | grep -oP "0.0.0.0:\K\d+" | head -1)
        elif [[ $name == *"minecraft_server_${stack_id}"* ]]; then
            minecraft_status="running"
            minecraft_port=$(echo "$ports" | grep -oP "0.0.0.0:\K\d+" | head -1)
        fi
    done <<< "$container_info"

    # Print status with address info when running
    echo -n "Stack $stack_id: SFTP server is $sftp_status"
    [ "$sftp_status" == "running" ] && echo -n " ($wan_ip:$sftp_port)"
    echo -n ", Minecraft server is $minecraft_status"
    [ "$minecraft_status" == "running" ] && echo -n " ($wan_ip:$minecraft_port)"
    echo
done