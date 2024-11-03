# Step 1: Build the application
FROM rust:latest AS builder

WORKDIR /usr/src/mc_stack
COPY . .

RUN cargo install --path .

# Step 2: Create a smaller image for the runtime
FROM debian:stable-slim

WORKDIR /mc_stack

# Install Docker CLI using the official Docker repository
RUN apt update && apt install -y \
    ca-certificates \
    curl \
    gnupg

# Add Docker's official GPG key
RUN install -m 0755 -d /etc/apt/keyrings && \
    curl -fsSL https://download.docker.com/linux/debian/gpg | gpg --dearmor -o /etc/apt/keyrings/docker.gpg && \
    chmod a+r /etc/apt/keyrings/docker.gpg

# Add the repository to Apt sources
RUN echo \
    "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/debian \
    $(. /etc/os-release && echo "$VERSION_CODENAME") stable" | \
    tee /etc/apt/sources.list.d/docker.list > /dev/null

# Install Docker CLI
RUN apt update && \
    apt install -y docker-ce-cli

# Clean up
RUN apt remove -y \
    ca-certificates \
    curl \
    gnupg \
    && \
    apt autoremove -y && \
    apt clean && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/mc_stack .

VOLUME ["/mc_stack/stacks"]

CMD ["./mc_stack"]