# Step 1: Build the application
FROM rust:slim AS builder

WORKDIR /usr/src/mc_stack

COPY . .

RUN cargo build --release

# Step 2: Download latest static docker CLI and compose in a separate stage
FROM debian:stable-slim AS docker-cli

RUN apt update && apt install -y curl

RUN DOCKER_VERSION=$(curl -s https://download.docker.com/linux/static/stable/x86_64/ | grep -o 'docker-[0-9]*\.[0-9]*\.[0-9]*\.tgz' | sort -V | tail -n 1 | sed 's/docker-//;s/\.tgz//') && \
    echo "Using Docker version: ${DOCKER_VERSION}" && \
    curl -fsSL "https://download.docker.com/linux/static/stable/x86_64/docker-${DOCKER_VERSION}.tgz" -o docker.tgz \
    && tar xzvf docker.tgz docker/docker \
    && mv docker/docker /usr/local/bin/docker \
    && rm -rf docker docker.tgz

RUN mkdir -p /usr/local/lib/docker/cli-plugins && \
    COMPOSE_VERSION=$(curl -s https://api.github.com/repos/docker/compose/releases/latest | grep -o '"tag_name": ".*"' | cut -d'"' -f4 | sed 's/^v//') && \
    echo "Using Docker Compose version: ${COMPOSE_VERSION}" && \
    curl -fsSL "https://github.com/docker/compose/releases/download/v${COMPOSE_VERSION}/docker-compose-linux-x86_64" -o /usr/local/lib/docker/cli-plugins/docker-compose && \
    chmod +x /usr/local/lib/docker/cli-plugins/docker-compose

# Step 3: Create a smaller image for the runtime
FROM debian:stable-slim

WORKDIR /mc_stack

COPY --from=docker-cli /usr/local/bin/docker /usr/local/bin/docker

COPY --from=docker-cli /usr/local/lib/docker/cli-plugins /usr/local/lib/docker/cli-plugins

COPY --from=builder /usr/src/mc_stack/target/release/mc_stack .

VOLUME ["/mc_stack/stacks"]

CMD ["./mc_stack"]