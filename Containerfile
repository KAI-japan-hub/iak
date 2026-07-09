# ------------------------------
# Stage 1. Build the app
# ------------------------------
FROM rust:1.83 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# ------------------------------
# Stage 2. Runtime image
# ------------------------------
FROM debian:trixie-slim

ARG GIT_REVISION
ARG BUILD_DATE
ARG VERSION

LABEL org.opencontainers.image.title="iak" \
      org.opencontainers.image.description="A color-coded ls for clearer file display" \
      org.opencontainers.image.url="https://github.com/KAI-japan-hub/iak" \
      org.opencontainers.image.source="https://github.com/KAI-japan-hub/iak" \
      org.opencontainers.image.version=${VERSION} \
      org.opencontainers.image.revision=${GIT_REVISION} \
      org.opencontainers.image.created=${BUILD_DATE} \
      org.opencontainers.image.licenses="CC0-1.0" \
      org.opencontainers.image.authors="Kai"

COPY --from=builder /app/target/release/iak /app/iak
WORKDIR /opt
ENTRYPOINT [ "/app/iak" ]
