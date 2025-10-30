# Dockerfile  (Hugging Face Spaces)
FROM python:3.11-slim

WORKDIR /app

# System deps (optional; remove curl if you don't need it)
RUN apt-get update && apt-get install -y curl && rm -rf /var/lib/apt/lists/*

# Copy deps first for layer caching
COPY requirements.txt .
RUN pip install --no-cache-dir --upgrade pip \
    && pip install --no-cache-dir -r requirements.txt

# Copy app
COPY app ./app

# Bind to the port provided by the platform
ENV PORT=7860
EXPOSE ${PORT}

# Optional healthcheck if you keep GET /health
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -fsS http://127.0.0.1:${PORT}/health || exit 1

CMD ["sh", "-c", "uvicorn app.main:app --host 0.0.0.0 --port ${PORT} --workers 1"]
