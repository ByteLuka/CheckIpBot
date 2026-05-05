FROM ghcr.io/astral-sh/uv:python3.14-bookworm-slim AS builder

WORKDIR /app
COPY pyproject.toml uv.lock ./
RUN uv sync --frozen --no-dev --no-cache


FROM python:3.14-slim-bookworm AS runtime

RUN adduser --disabled-password --gecos "" botuser

WORKDIR /app
COPY --from=builder /app/.venv /app/.venv
COPY bot.py ./
COPY extensions/ ./extensions/

ENV PATH="/app/.venv/bin:$PATH" \
    PYTHONUNBUFFERED=1 \
    PYTHONDONTWRITEBYTECODE=1

USER botuser
CMD ["python", "bot.py"]
