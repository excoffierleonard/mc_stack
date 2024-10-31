#!/bin/bash

format_json() {
    printf '{"message":"%s"}\n' "$1"
}

format_error() {
    echo "{\"message\": \"$1\", \"timestamp\": \"$(date -u +"%Y-%m-%dT%H:%M:%SZ")\"}" >&2
    exit 1
}

[ -z "$1" ] && format_error "Usage: $0 <stack_id>"

stack_id=$1
stack_compose_file="$(dirname "$(realpath "$0")")/../stacks/stack_$stack_id/compose.yaml"

[ ! -f "$stack_compose_file" ] && format_error "Stack $stack_id does not exist"

docker compose -f "$stack_compose_file" down > /dev/null 2>&1 || format_error "Failed to stop stack $stack_id"

format_json "Stack $stack_id has been successfully stopped"