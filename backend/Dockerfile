# ---------- Build stage ----------
# Use a newer Rust version that supports edition 2024
FROM rust:1.82.0 as builder

WORKDIR /app

COPY . .

# Cache dependencies
RUN cargo build --release
RUN rm -rf src

# Install minimal build deps
RUN apt-get update \
 && apt-get install -y --no-install-recommends pkg-config libssl-dev ca-certificates \
 && rm -rf /var/lib/apt/lists/*

# Copy dependency files first for better caching
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src && echo "fn main() {}" > src/main.rs

# Cache dependencies
RUN cargo build --release
RUN rm -rf src

# Copy actual source code
COPY src ./src

# Clean and rebuild with actual code
RUN cargo clean
RUN cargo build --release

# ---------- Runtime stage ----------
FROM debian:bookworm-slim
WORKDIR /app

RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates \
 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/solana-defi-backend /usr/local/bin/backend

ENV PORT=${PORT:-8080}
ENV AI_BASE_URL="https://rejwar-solana-defi-ai.hf.space"
ENV RUST_LOG=info
EXPOSE ${PORT}

CMD ["/usr/local/bin/backend"]