#!/bin/bash

format_json() {
    local msg=$1
    printf '{"message":"%s"}\n' "$msg"
}

if [ -z "$1" ]; then
    format_json "Usage: $0 <stack_id>" >&2
    exit 1
fi

stack_id=$1
base_dir="$(dirname "$(realpath "$0")")/../stacks"
stack_dir="$base_dir/stack_$stack_id"
stack_compose_file="$stack_dir/compose.yaml"

if [ ! -f "$stack_compose_file" ]; then
    format_json "Stack $stack_id does not exist" >&2
    exit 1
fi

if ! docker compose -f "$stack_compose_file" up -d; then
    format_json "Failed to start stack $stack_id" >&2
    exit 1
fi

format_json "Stack $stack_id has been successfully started"
