# Step 1: Build the application
FROM rust:latest AS builder

WORKDIR /usr/src/mc_stack
COPY . .

RUN cargo install --path .

# Step 2: Create a smaller image for the runtime
FROM debian:stable-slim

WORKDIR /mc_stack

RUN apt-get update && apt-get install -y curl

RUN curl -fsSL https://get.docker.com -o get-docker.sh && sh get-docker.sh

COPY --from=builder /usr/local/cargo/bin/mc_stack .

VOLUME [ "./stacks" ]

CMD ["./mc_stack"]