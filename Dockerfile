# Multi-stage build for Rust application
FROM rust:1.70-bullseye as builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    python3 \
    python3-pip \
    python3-dev \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy dependency files
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src
COPY migrations ./migrations
COPY config ./config

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    python3 \
    python3-pip \
    libssl1.1 \
    && rm -rf /var/lib/apt/lists/*

# Install Python ML dependencies
RUN pip3 install numpy pandas scikit-learn

# Create app user
RUN useradd -m appuser

# Set working directory
WORKDIR /app

# Copy binary from builder stage
COPY --from=builder /app/target/release/book_recommendation_backend /usr/local/bin/
COPY --from=builder /app/migrations ./migrations
COPY --from=builder /app/config ./config

# Create directories for models and uploads
RUN mkdir -p models python_models logs uploads && \
    chown -R appuser:appuser /app

# Switch to non-root user
USER appuser

# Expose port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Start the application
CMD ["book_recommendation_backend"]