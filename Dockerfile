FROM rust:1.73-bookworm as builder

RUN apt update && apt -yq upgrade
RUN apt -yq install libpq-dev libssl-dev musl-tools

WORKDIR /
RUN cargo new app
WORKDIR /app

RUN rustup target add x86_64-unknown-linux-musl

COPY Cargo.toml Cargo.lock ./
RUN cargo build --target x86_64-unknown-linux-musl --release

COPY . .
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:3.18
LABEL org.opencontainers.image.source=https://github.com/aicacia/rs-auth

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/object_storage /usr/local/bin
ENV RUN_MODE=production
RUN touch .env

CMD ["object_storage"]
