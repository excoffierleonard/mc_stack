#!/bin/bash

base_dir="$(dirname "$(realpath "$0")")/../stacks"

# Check if the stacks directory exists
if [ ! -d "$base_dir" ]; then
    echo "Error: Stacks directory $base_dir does not exist."
    exit 1
fi

# List all stack directories
stack_dirs=$(find "$base_dir" -maxdepth 1 -type d -name 'stack_*')

if [ -z "$stack_dirs" ]; then
    echo "No stacks found."
    exit 0
fi

# Iterate over each stack directory and check its status
for stack_dir in $stack_dirs; do
    stack_id=$(basename "$stack_dir" | cut -d'_' -f2)
    stack_compose_file="$stack_dir/compose.yaml"

    if [ ! -f "$stack_compose_file" ]; then
        echo "Stack $stack_id: compose.yaml not found."
        continue
    fi

    # Check if the stack is running
    stack_status=$(docker compose -f "$stack_compose_file" ps -q | xargs docker inspect -f '{{.State.Running}}' 2>/dev/null | grep true >/dev/null && echo "running" || echo "stopped")

    echo "Stack $stack_id: $stack_status"
done