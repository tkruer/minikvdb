# Use the official Rust image as the build environment
FROM rust:1.84 as builder

# Create and change to the app directory
WORKDIR /usr/src/minikvdb

# Copy the project files
COPY . .

# Build the minikvdb-server binary in release mode
RUN cargo build --release --bin minikvdb-server

# Use a minimal base image for the runtime
FROM debian:buster-slim

# Install any necessary runtime dependencies (if needed)
RUN apt-get update && \
  apt-get install -y libssl1.1 ca-certificates && \
  rm -rf /var/lib/apt/lists/*

# Set the working directory for the runtime image
WORKDIR /usr/local/bin

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/minikvdb/target/release/minikvdb-server .

# Expose the default port (6379)
EXPOSE 6379

# Set the entrypoint to run the server
CMD ["./minikvdb-server"]
