#!/bin/bash

# Check if stack_id is provided
if [ -z "$1" ]; then
    echo "Usage: $0 <stack_id>"
    exit 1
fi

stack_id=$1
stack_dir="stack_$stack_id"
stack_compose_file="$stack_dir/compose.yaml"

# Check if the stack directory exists
if [ ! -d "$stack_dir" ]; then
    echo "Error: Stack directory $stack_dir does not exist."
    exit 1
fi

# Start the Docker containers using docker compose -f
docker compose -f "$stack_compose_file" up -d

# Echo the success message
echo "Stack $stack_id has been successfully started."
