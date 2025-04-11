# Step 1: Build stage with latest stable Rust
FROM rust:1.81.0-slim AS builder

WORKDIR /app
COPY . .

# Fetch dependencies and build
RUN cargo fetch
RUN cargo build --release

# Step 2: Minimal runtime image
FROM debian:bookworm-slim

# Install only what's needed for networking and TLS roots
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates netbase && \
    rm -rf /var/lib/apt/lists/*

# Copy the compiled binary
COPY --from=builder /app/target/release/relayer /usr/local/bin/relayer

ENTRYPOINT ["relayer"]
