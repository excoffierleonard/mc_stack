# Step 1: Build the application with musl target
FROM rust:alpine AS builder

# Install build dependencies
RUN apk add --no-cache musl-dev

WORKDIR /usr/src/mc_stack

# Add musl target
RUN rustup target add x86_64-unknown-linux-musl

COPY . .

# Build statically linked binary
RUN cargo build --target x86_64-unknown-linux-musl --release

# Step 2: Create final image
FROM alpine

# Install Docker CLI and compose from Alpine packages
RUN apk add --no-cache docker-cli docker-compose

WORKDIR /mc_stack

# Copy the musl binary from builder
COPY --from=builder /usr/src/mc_stack/target/x86_64-unknown-linux-musl/release/mc_stack .

VOLUME ["/mc_stack/stacks"]

CMD ["./mc_stack"]