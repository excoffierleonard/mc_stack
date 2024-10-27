#!/bin/bash

# Check if stack_id is provided
if [ -z "$1" ]; then
    echo "Usage: $0 <stack_id>" >&2
    exit 1
fi

stack_id=$1
base_dir="$(dirname "$(realpath "$0")")/../stacks"
stack_dir="$base_dir/stack_$stack_id"
stack_compose_file="$stack_dir/compose.yaml"

# Check if the stack directory exists
if [ ! -d "$stack_dir" ]; then
    echo "Error: Stack directory $stack_dir does not exist." >&2
    exit 1
fi

# Stop the Docker containers using docker compose -f
docker compose -f "$stack_compose_file" down

# Echo the success message
echo "Stack $stack_id has been successfully stopped."
