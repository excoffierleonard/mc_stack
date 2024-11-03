# Step 1: Build the application
FROM rust:latest AS builder

WORKDIR /usr/src/mc_stack
COPY . .

RUN cargo install --path .

# Step 2: Create a smaller image for the runtime
FROM debian:stable-slim

COPY --from=builder /usr/local/cargo/bin/mc_stack /usr/local/bin/mc_stack

CMD ["mc_stack"]