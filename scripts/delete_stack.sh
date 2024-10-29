#!/bin/bash

format_json() {
    printf '{"message":"%s"}\n' "$1"
}

[ -z "$1" ] && format_json "Usage: $0 <stack_id>" >&2 && exit 1

stack_id=$1
stack_compose_file="$(dirname "$(realpath "$0")")/../stacks/stack_$stack_id/compose.yaml"

[ ! -f "$stack_compose_file" ] && format_json "Stack $stack_id does not exist" >&2 && exit 1

docker compose -f "$stack_compose_file" down > /dev/null 2>&1 || { format_json "Failed to stop stack $stack_id" >&2; exit 1; }

docker volume rm "minecraft_server_${stack_id}" > /dev/null 2>&1 || { format_json "Failed to remove minecraft server volume" >&2; exit 1; }

rm -rf "$(dirname "$stack_compose_file")" > /dev/null 2>&1 || { format_json "Failed to remove stack directory" >&2; exit 1; }

format_json "Stack $stack_id has been successfully deleted"
