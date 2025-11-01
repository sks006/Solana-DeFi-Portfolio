# ---------- Runtime stage ----------
FROM debian:bookworm-slim
WORKDIR /app

RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates \
 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/solana-defi-backend /usr/local/bin/backend
COPY start.sh /usr/local/bin/start.sh
RUN chmod +x /usr/local/bin/start.sh

ENV PORT=3000
ENV AI_BASE_URL="https://rejwar-solana-defi-ai.hf.space"
ENV RUST_LOG=info
EXPOSE 3000

CMD ["/usr/local/bin/start.sh"]