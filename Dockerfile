# syntax=docker/dockerfile:1

ARG RUST_VERSION=1.82
ARG APP_NAME=telegram-dice-bot

FROM rust:${RUST_VERSION}-slim AS builder
ARG APP_NAME
WORKDIR /app

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src && echo "fn main(){}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Build application and install directly (use --locked to prevent dependency updates)
COPY . .
RUN cargo install --path . --root /usr/local --locked

FROM debian:bookworm-slim AS runtime
ENV PORT=5000

# Create non-root user
RUN useradd -u 10001 -m appuser

WORKDIR /app
COPY --from=builder /usr/local/bin/${APP_NAME} /usr/local/bin/${APP_NAME}

EXPOSE 5000
USER root

CMD ["/bin/sh", "-c", "echo 'Starting bot...' && /usr/local/bin/telegram-dice-bot"]
