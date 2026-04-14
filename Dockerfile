FROM rust:1.86-alpine AS builder

WORKDIR /app

RUN apk add --no-cache \
    musl-dev \
    pkgconfig \
    openssl-dev

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/app/target \
    cargo build --release --locked && cp /app/target/release/musiseerr /app/musiseerr

FROM alpine:3.22

WORKDIR /app

RUN apk add --no-cache \
    ca-certificates \
    libgcc \
    libssl3

COPY --from=builder /app/musiseerr /app/musiseerr

RUN adduser -D -u 10001 appuser
USER appuser

EXPOSE 3000

CMD ["/app/musiseerr"]