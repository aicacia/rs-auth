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

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/auth /usr/local/bin

ENV RUN_MODE=production

RUN echo DATABASE_URL=postgres://postgres:postgres@localhost:5432/auth >> .env

CMD ["auth"]
