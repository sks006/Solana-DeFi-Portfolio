FROM rust:1.81-slim AS builder

WORKDIR /app
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/solana-defi-backend /app/solana-defi-backend
EXPOSE 8080
CMD ["./solana-defi-backend"]
