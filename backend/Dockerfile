# ---------- Build stage ----------
FROM rust:1.81-slim AS builder
WORKDIR /app

RUN apt-get update \
 && apt-get install -y --no-install-recommends pkg-config libssl-dev ca-certificates \
 && rm -rf /var/lib/apt/lists/*

# Remove base64ct from dependencies before building
COPY Cargo.toml Cargo.lock ./
RUN sed -i '/base64ct/d' Cargo.toml

COPY src ./src

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