# DOCKERFILE FOR RUST SERVER
# VERSION 1.0.0

# Build binary
FROM rust:1.64.0 as builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path .


# Run binary
FROM debian:buster-slim
WORKDIR /usr/src/myapp
COPY --from=builder /usr/local/cargo/bin/rust-server .
CMD ["./rust-server"]


