# Dockerfile (Hugging Face Spaces)
FROM python:3.11-slim
WORKDIR /app

RUN apt-get update && apt-get install -y curl && rm -rf /var/lib/apt/lists/*

# ⬅️ correct paths (note the ai/ prefix)
COPY ai/python/requirements.txt ./requirements.txt
RUN pip install --no-cache-dir --upgrade pip \
    && pip install --no-cache-dir -r requirements.txt

# ⬅️ copy your FastAPI app (it lives in ai/python/app)
COPY ai/python/app ./app

ENV PORT=7860
EXPOSE ${PORT}

HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
  CMD curl -fsS http://127.0.0.1:${PORT}/health || exit 1

CMD ["sh","-c","uvicorn app.main:app --host 0.0.0.0 --port ${PORT} --workers 1"]
