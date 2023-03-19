# DOCKERFILE FOR RUST SERVER
# VERSION 1.0.0

# Build binary
FROM rust:1.64.0 as builder
WORKDIR /usr/src/myapp
COPY . .
RUN RUSTFLAGS="-C target-feature=-crt-static" cargo install --path .



# Run binary
FROM debian:bullseye-slim
WORKDIR /usr/src/myapp
# Install necessary dependencies
RUN apt-get update && \
    apt-get install -y libssl1.1 && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/rust-server .
CMD ["./rust-server"]


