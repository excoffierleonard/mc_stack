#!/bin/bash

format_json() {
    local stacks=$1
    cat << EOF
{
    "message": "Stack status retrieved successfully",
    "data": {
        "wan_ip": "$(wget -qO- http://ipinfo.io/ip)",
        "stacks": [
            $stacks
        ]
    }
}
EOF
}

format_error() {
    local msg=$1
    echo "{\"message\": \"$msg\", \"timestamp\": \"$(date -u +"%Y-%m-%dT%H:%M:%SZ")\"}" >&2
    exit 1
}

base_dir="$(dirname "$(realpath "$0")")/../stacks"

# Check if base directory exists
[ ! -d "$base_dir" ] && format_error "Stacks directory $base_dir does not exist"

# Find all valid compose files as source of truth
compose_files=$(find "$base_dir" -name "compose.yaml" -type f | sort)
[ -z "$compose_files" ] && format_json "[]" && exit 0

# Get all running containers info once
container_info=$(docker ps --format '{{.Names}}|{{.Ports}}')

# Initialize stacks array
stacks_json=""
separator=""

# Process each stack
while IFS= read -r compose_file; do
    stack_dir=$(dirname "$compose_file")
    stack_id=$(basename "$stack_dir" | cut -d'_' -f2)
    
    # Initialize service status
    sftp_status="stopped"
    sftp_port=""
    minecraft_status="stopped"
    minecraft_port=""
    
    # Check running services
    while IFS='|' read -r name ports || [ -n "$name" ]; do
        if [[ $name == *"sftp_server_${stack_id}"* ]]; then
            sftp_status="running"
            sftp_port=$(echo "$ports" | grep -oP "0.0.0.0:\K\d+" | head -1)
        elif [[ $name == *"minecraft_server_${stack_id}"* ]]; then
            minecraft_status="running"
            minecraft_port=$(echo "$ports" | grep -oP "0.0.0.0:\K\d+" | head -1)
        fi
    done <<< "$container_info"
    
    # Build JSON for this stack
    stack_json="{
        \"stack_id\": \"$stack_id\",
        \"services\": {
            \"sftp\": {
                \"status\": \"$sftp_status\"
                $([ -n "$sftp_port" ] && echo ", \"port\": \"$sftp_port\"")
            },
            \"minecraft\": {
                \"status\": \"$minecraft_status\"
                $([ -n "$minecraft_port" ] && echo ", \"port\": \"$minecraft_port\"")
            }
        }
    }"
    
    stacks_json="${stacks_json}${separator}${stack_json}"
    separator=","
done <<< "$compose_files"

# Output final JSON
format_json "$stacks_json"