FROM rust:1-buster AS builder
WORKDIR /app
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    --mount=type=cache,target=/root/.cache/prisma/binaries \
    cargo prisma generate && cargo build --release \
    && mkdir -p /dist \
    && cp -r /app/target/release/server /dist \
    && cp -r /app/prisma /dist/

FROM debian:buster AS runtime
RUN apt-get update && apt install -y openssl
WORKDIR /app
COPY --from=builder /dist/server /usr/local/bin
COPY --from=builder /dist/prisma /app/prisma
ENTRYPOINT ["/usr/local/bin/server"]
