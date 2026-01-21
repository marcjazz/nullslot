# Use the official Rust image as a builder
FROM rust:bookworm AS builder

WORKDIR /usr/src/app

# Install dependencies for building
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy the source code
COPY . .

# Build the application
ENV SQLX_OFFLINE=true
RUN cargo build --release

# Use a slim image for the runtime
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN groupadd -r nullslot && useradd --no-log-init -r -g nullslot nullslot
USER nullslot

WORKDIR /usr/local/bin

# Copy the binary from the builder
COPY --from=builder /usr/src/app/target/release/backend .
# Copy migrations for startup
COPY --from=builder /usr/src/app/migrations ./migrations

# Expose the port
EXPOSE 8080

# Healthcheck
HEALTHCHECK --interval=30s --timeout=10s --retries=3 CMD curl --fail http://localhost:8080/api/v1/health || exit 1

# Run the binary
CMD ["./backend"]
