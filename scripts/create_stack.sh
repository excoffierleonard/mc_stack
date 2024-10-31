#!/bin/bash

# Constants
MAX_STACKS=8
INCREMENT=3

format_json() {
    local stack_id=$1
    local server_port=$2
    local rcon_port=$3
    local sftp_port=$4
    cat << EOF
{
    "message": "Stack $stack_id has been successfully created",
    "data": {
        "stack_id": "$stack_id",
        "ports": {
            "minecraft_server": "$server_port",
            "rcon": "$rcon_port",
            "sftp_server": "$sftp_port"
        }
    }
}
EOF
}

format_error() {
    echo "{\"message\": \"$1\"}" >&2
    exit 1
}

# Setup base directories
base_dir="$(dirname "$(realpath "$0")")/.."
stacks_dir="$base_dir/stacks"
template_dir="$base_dir/template"
template_env="$template_dir/.env"

# Validate required files and directories
[ ! -d "$stacks_dir" ] && format_error "Stacks directory does not exist"
[ ! -f "$template_env" ] && format_error "Template .env file not found"
[ ! -f "$template_dir/compose.yaml" ] && format_error "Template compose.yaml not found"

# Count existing stacks based on compose files as source of truth
stack_count=$(find "$stacks_dir" -name "compose.yaml" -type f | wc -l)
[ "$stack_count" -ge "$MAX_STACKS" ] && format_error "Maximum number of stacks ($MAX_STACKS) reached"

# Get base ports from template
base_server_port=$(grep '^SERVER_PORT=' "$template_env" | cut -d '=' -f 2)
base_rcon_port=$(grep '^RCON_PORT=' "$template_env" | cut -d '=' -f 2)
base_sftp_port=$(grep '^SFTP_SERVER_PORT=' "$template_env" | cut -d '=' -f 2)

# Find next available stack ID
declare -A used_ids
for dir in "$stacks_dir"/stack_*/; do
    [[ -f "$dir/compose.yaml" ]] && used_ids[${dir#*stack_}]=1
done

new_stack_id=1
while [ ${used_ids[$new_stack_id]} ]; do
    ((new_stack_id++))
    [ "$new_stack_id" -gt "$MAX_STACKS" ] && format_error "No available stack IDs"
done

# Setup new stack
new_stack_dir="$stacks_dir/stack_$new_stack_id"
new_stack_env="$new_stack_dir/.env"

# Calculate new ports
new_server_port=$((base_server_port + new_stack_id * INCREMENT))
new_rcon_port=$((base_rcon_port + new_stack_id * INCREMENT))
new_sftp_port=$((base_sftp_port + new_stack_id * INCREMENT))

# Create directory and copy templates
mkdir -p "$new_stack_dir" || format_error "Failed to create stack directory"
cp "$template_dir/compose.yaml" "$template_env" "$new_stack_dir/" || format_error "Failed to copy template files"

# Update .env file
sed -i \
    -e "s/^MINECRAFT_SERVER_SERVICE=.*/MINECRAFT_SERVER_SERVICE=minecraft_server_$new_stack_id/" \
    -e "s/^MINECRAFT_SERVER_VOLUME=.*/MINECRAFT_SERVER_VOLUME=minecraft_server_$new_stack_id/" \
    -e "s/^MINECRAFT_SERVER_NETWORK=.*/MINECRAFT_SERVER_NETWORK=minecraft_server_$new_stack_id/" \
    -e "s/^SERVER_PORT=.*/SERVER_PORT=$new_server_port/" \
    -e "s/^RCON_PORT=.*/RCON_PORT=$new_rcon_port/" \
    -e "s/^SFTP_SERVER_PORT=.*/SFTP_SERVER_PORT=$new_sftp_port/" \
    -e "s/^SFTP_SERVER_SERVICE=.*/SFTP_SERVER_SERVICE=sftp_server_$new_stack_id/" \
    "$new_stack_env" || format_error "Failed to update .env file"

# Start the containers
if ! docker compose -f "$new_stack_dir/compose.yaml" up -d; then
    rm -rf "$new_stack_dir"
    format_error "Failed to start containers. Stack creation rolled back"
fi

# Output success JSON
format_json "$new_stack_id" "$new_server_port" "$new_rcon_port" "$new_sftp_port"