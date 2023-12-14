# Build binary
FROM rust:1.64.0 as builder
WORKDIR /usr/src/myapp

# Copy the manifest files first to cache the dependencies
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Build the application
RUN cargo build --release

# Run binary
FROM debian:bullseye-slim
WORKDIR /app

# Install runtime dependencies, if necessary
RUN apt-get update && \
    apt-get install -y libssl1.1 && \
    rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/myapp/target/release/rust-server .

# Non-root user (optional but recommended)
# Consider adding user creation steps here if you decide to use a non-root user

CMD ["./rust-server"]
