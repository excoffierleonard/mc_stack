# Minecraft Server Stack

## Getting Started

```bash
git clone https://git.jisoonet.com/el/mc_stack.git && \
cd mc_stack
```

## Dev

```bash
cargo run
```

## Prod

### Build and Run

```bash
cargo build --release && \
pkill mc_stack && \
nohup target/release/mc_stack &> output.log &
```

### Stop

```bash
pkill mc_stack
```

### Docker

```bash
docker compose pull && \
docker compose down && \
docker compose up -d
```

## Todo

- Implement a backup mechanism using duplicacy
- Migrate everything to rust, the static web files may be converted to webassembly
- Directly using Docker Api to manage containers rather than installing docker cli, and maybe in the far future implement the container management system fully in rust (maybe not docker compose is usefull)
- Use docker hashes as a better source of truth for listing container
- List all containers and their status using docker ps rather than weird combination of listing the dirs etc...
- Really do better introspection of the avaible docker commands. Maybe use the docker crate to do this

## Notes

- The service runs on `0.0.0.0:8080`
- The docker container need to have the `docker.sock` mounted to `/var/run/docker.sock`
