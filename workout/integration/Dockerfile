# Build stage
FROM rust:latest AS builder

WORKDIR /app

# Copy workspace root files
COPY Cargo.toml Cargo.lock ./

# Copy all workspace members needed for integration
COPY integration ./integration
COPY business ./business
COPY entity ./entity
COPY migration ./migration
COPY application ./application
COPY src ./src

# Build integration binary in release mode
RUN cargo build --release --package integration

# Runtime stage - must match the builder's glibc (rust:latest is Debian
# trixie / glibc 2.41); bookworm-slim (glibc 2.36) is too old and makes the
# compiled binary fail at startup with a GLIBC_2.38 not found error.
FROM debian:trixie-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    netcat-openbsd \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/integration /usr/local/bin/integration
#COPY /certs/server.crt /etc/grpc/certs/server.crt
#RUN chmod 644 /etc/grpc/certs/server.crt
#COPY /certs/server.key /etc/grpc/certs/server.key
#RUN chmod 600 /etc/grpc/certs/server.key

EXPOSE 50051

# TCP-connect liveness check - the gRPC listener has no HTTP health endpoint,
# so this just confirms the process is up and accepting connections.
HEALTHCHECK --interval=30s --timeout=10s --start-period=40s --retries=3 \
    CMD nc -z localhost ${GRPC_PORT:-50051} || exit 1

CMD ["integration"]

