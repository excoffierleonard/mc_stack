#!/bin/bash

format_json() {
    printf '{"message":"%s"}\n' "$1"
}

[ -z "$1" ] && format_json "Usage: $0 <stack_id>" >&2 && exit 1

stack_id=$1
stack_compose_file="$(dirname "$(realpath "$0")")/../stacks/stack_$stack_id/compose.yaml"

[ ! -f "$stack_compose_file" ] && format_json "Stack $stack_id does not exist" >&2 && exit 1

docker compose -f "$stack_compose_file" up -d > /dev/null 2>&1 || { format_json "Failed to start stack $stack_id" >&2; exit 1; }

format_json "Stack $stack_id has been successfully started"