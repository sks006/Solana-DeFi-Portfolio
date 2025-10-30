# ---------- Build stage ----------
FROM rust:1.81-slim AS builder
WORKDIR /app

# Install minimal build deps (openssl headers are safe to include even if you use rustls)
RUN apt-get update \
 && apt-get install -y --no-install-recommends pkg-config libssl-dev ca-certificates \
 && rm -rf /var/lib/apt/lists/*

# Cache deps first
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release || true

# Rebuild with real sources (the copy above already brought src in; we keep it simple)
RUN cargo build --release

# ---------- Runtime stage ----------
FROM debian:bookworm-slim
WORKDIR /app

RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates \
 && rm -rf /var/lib/apt/lists/*

# Copy optimized binary
COPY --from=builder /app/target/release/solana-defi-backend /usr/local/bin/backend

# Dynamic port (Render/Railway/Fly inject $PORT)
ENV PORT=${PORT:-8080}
ENV AI_BASE_URL="https://rejwar-solana-defi-ai.hf.space"
ENV RUST_LOG=info
EXPOSE ${PORT}

# If you have runtime config files:
# COPY config/ /app/config/

CMD ["/usr/local/bin/backend"]
