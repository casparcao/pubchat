# Use Rust official image as build environment
FROM rust:1.88 as builder

# Create app directory
WORKDIR /app

# Copy project files
COPY . .

# Build the application in release mode
RUN cargo build --release

# Use Debian slim image for runtime
FROM debian:bullseye-slim

# Install ca-certificates for SSL connections
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/user ./user

# Copy environment file
COPY --from=builder /app/.env ./.env

# Expose port
EXPOSE 3000

# Run the application
CMD ["./user"]