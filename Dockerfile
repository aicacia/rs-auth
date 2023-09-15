FROM rust:1.72-bookworm as builder

RUN apt update && apt -yq upgrade
RUN apt -yq install libpq-dev

WORKDIR /
RUN cargo new app
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

COPY . .
RUN cargo build --release

FROM debian:bookworm

WORKDIR /app

COPY --from=builder /app/target/release/auth /usr/local/bin

ENV RUN_MODE=production

CMD ["auth"]
