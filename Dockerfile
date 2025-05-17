# -------- Build Stage --------
FROM rust:1.76 as builder

# Create a non-root user for safety
RUN useradd -m appuser

WORKDIR /app

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf src

# Copy full source and build
COPY . .
RUN cargo build --release

# -------- Runtime Stage --------
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/zerobyte /app/zerobyte

# Use non-root user
RUN useradd -m appuser
USER appuser

EXPOSE 8080

# Start the app
CMD ["./zerobyte"]