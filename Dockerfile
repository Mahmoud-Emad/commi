# Multi-stage build for optimal image size
FROM rust:1.75-slim as builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    git \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build for release
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    git \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -r -s /bin/false commi

# Copy the binary from builder stage
COPY --from=builder /app/target/release/commi /usr/local/bin/commi

# Set ownership and permissions
RUN chown root:root /usr/local/bin/commi && \
    chmod 755 /usr/local/bin/commi

# Switch to non-root user
USER commi

# Set working directory
WORKDIR /workspace

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD commi --version || exit 1

# Default command
ENTRYPOINT ["commi"]
CMD ["--help"]
