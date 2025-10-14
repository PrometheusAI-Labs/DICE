# syntax=docker/dockerfile:1

ARG RUST_VERSION=1.79
ARG APP_NAME=telegram-dice-bot

FROM rust:${RUST_VERSION}-slim AS builder
WORKDIR /app

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src && echo "fn main(){}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Build application
COPY . .
RUN cargo build --release --bin ${APP_NAME}

FROM debian:bookworm-slim AS runtime
ENV RUST_LOG=info
ENV PORT=5000

# Create non-root user
RUN useradd -u 10001 -m appuser

WORKDIR /app
COPY --from=builder /app/target/release/${APP_NAME} /usr/local/bin/${APP_NAME}

EXPOSE 5000
USER appuser

CMD ["/usr/local/bin/telegram-dice-bot"]
