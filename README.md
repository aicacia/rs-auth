# Aicacia Auth API

[![license](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue")](LICENSE-MIT)
[![Test Status](https://github.com/aicacia/rs-auth/workflows/Tests/badge.svg?event=push)](https://github.com/nathanfaucett/rs-auth/actions)

aicacia auth api

## Dev

- install [rustup](https://rustup.rs/)
- install [cargo-watch](https://crates.io/crates/cargo-watch)
- rename .env file `cp .env.example .env`
- Startup main web service `cargo watch -c -w src -x run`
- Local mailer (Optional) `docker run --name=mailhog -p 25:1025 -p 8025:8025 --rm mailhog/mailhog`
- create postgres `docker compose -f postgres-docker-compose.yaml up -d`
- delete postgres `docker compose -f postgres-docker-compose.yaml down` and `docker volume rm auth_auth-postgres`

## Build

- `cargo install --path .`

## Migrations

- create the database `sqlx database create`
- run migrations `sqlx migrate run`
- prepare for offline `cargo sqlx prepare`

## Extra DB Commands

- drop the database `sqlx database drop`
- revert migrations `sqlx migrate revert`
