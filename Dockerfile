# Step 1: Build stage with musl for static linking
FROM rust:1.81.0-slim AS builder

WORKDIR /app
COPY . .

# Install the musl target
RUN rustup target add x86_64-unknown-linux-musl

# Fetch dependencies and build with musl target
RUN cargo fetch
RUN cargo build --release --target x86_64-unknown-linux-musl

# Step 2: Runtime stage using scratch (empty base)
FROM scratch

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/relayer /relayer

ENTRYPOINT ["/relayer"]
