# Use the official Rust image as a builder
FROM rust:1.84-slim AS builder

WORKDIR /usr/src/app

# Install dependencies for building
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy the source code
COPY . .

# Build the application
RUN cargo build --release

# Use a slim image for the runtime
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin

# Copy the binary from the builder
COPY --from=builder /usr/src/app/target/release/backend .
# Copy migrations for startup
COPY --from=builder /usr/src/app/migrations ./migrations

# Expose the port
EXPOSE 8080

# Run the binary
CMD ["./backend"]
