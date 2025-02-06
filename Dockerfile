# ====== 1) Builder Stage ======
FROM rust:1.81 as builder

WORKDIR /app
COPY . .

# Install SQLx CLI for migrations
RUN cargo install sqlx-cli

# Build in release mode
RUN cargo build --release

# ====== 2) Final Stage (Slim) ======
FROM debian:bullseye-slim

# Install runtime dependencies (SSL, certs, etc.)
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the compiled binary
COPY --from=builder /app/target/release/rust4u-backend /app/rust4u-backend

# Copy migrations folder
COPY --from=builder /app/migrations /app/migrations

# Copy the sqlx binary from builder to final image
COPY --from=builder /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx

# If you have a .env or other configs needed at runtime, copy them too:
# COPY .env /app/.env

# By default, run migrations, then start the backend
CMD ["/bin/bash", "-c", "sqlx migrate run && /app/rust4u-backend"]
