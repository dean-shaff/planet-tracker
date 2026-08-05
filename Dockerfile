FROM lukemathwalker/cargo-chef:latest-rust-trixie AS chef
WORKDIR /app/client
RUN cargo install trunk --locked
RUN rustup target add wasm32-unknown-unknown

FROM chef AS planner
COPY ./client /app/client
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/client/recipe.json recipe.json
# This layer is cached as long as Cargo.toml/Cargo.lock don't change
RUN cargo chef cook --release --target wasm32-unknown-unknown --recipe-path recipe.json

COPY ./client /app/client
RUN trunk build --release

FROM python:3.12.13-slim-trixie
COPY --from=ghcr.io/astral-sh/uv:0.6.17 /uv /uvx /bin/

RUN apt-get update && apt-get install -y libssl-dev build-essential ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY uv.lock pyproject.toml app.py ./
RUN uv sync --frozen --no-dev

COPY --from=builder /app/client/dist /app/dist

CMD ["sh", "-c", "uv run granian --interface asgi app:app --host 0.0.0.0 --port ${PORT:-8080}"]