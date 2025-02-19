FROM rust:bookworm AS builder

WORKDIR /app

RUN --mount=type=bind,source=config,target=config \
    --mount=type=bind,source=crates,target=crates \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    --mount=type=bind,source=.cargo,target=.cargo \
    cargo build  -p suzhaobao --release \
    && cp ./target/release/suzhaobao /bin/suzhaobao \
    && cp -r ./config /bin/config


FROM debian:bookworm-slim

RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends ca-certificates openssl

WORKDIR /app


COPY --from=builder /bin/suzhaobao ./
COPY --from=builder /bin/config ./config

EXPOSE 5800
CMD ["/app/suzhaobao"]
