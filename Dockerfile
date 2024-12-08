FROM rust:1.82-bookworm as rust

RUN apt update && apt -yq upgrade
RUN apt -yq install musl-tools libpq-dev

RUN rustup default nightly
RUN rustup target add x86_64-unknown-linux-musl
RUN rustup update

WORKDIR /

FROM rust as deps

WORKDIR /
RUN cargo new app && touch /app/src/lib.rs
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM rust as builder

WORKDIR /app

COPY --from=deps /app/target/ /app/target/
COPY . .
RUN touch /app/src/lib.rs && touch /app/src/main.rs
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:3.21
LABEL org.opencontainers.image.source=https://github.com/aicacia/rs-auth

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/auth /usr/local/bin

ENV RUN_MODE=production

CMD ["auth"]
