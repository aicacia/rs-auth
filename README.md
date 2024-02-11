# Aicacia Auth API

[![license](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue")](LICENSE-MIT)
[![Test Status](https://github.com/aicacia/rs-auth/workflows/Tests/badge.svg?event=push)](https://github.com/nathanfaucett/rs-auth/actions)

aicacia auth api

## Dev

- install [rustup](https://rustup.rs/)
- install [cargo-watch](https://crates.io/crates/cargo-watch)
- install [sqlx-cli](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli)
- rename .env file `cp .env.example .env`
- Startup main web service `cargo watch -c -w src -x run`
- View [OpenApi Docs](https://petstore.swagger.io/?url=http://localhost:8080/openapi.json)
- Local mailer (Optional) `docker run --name=mailhog -p 25:1025 -p 8025:8025 --rm mailhog/mailhog`
- create services `docker compose up -d`
- delete services `docker compose down` and `docker volume rm auth_postgres`

## Build

- `cargo install --path .`

## Migrations

- create the database `sqlx database create`
- run migrations `sqlx migrate run`
- prepare for offline `cargo sqlx prepare`

## Extra DB Commands

- drop the database `sqlx database drop`
- revert migrations `sqlx migrate revert`

## Docker/Helm

### Deploy

- `docker build -t ghcr.io/aicacia/auth-api:latest .`
- `docker push ghcr.io/aicacia/auth-api:latest`
- `helm upgrade auth helm/auth-api -n api --install -f values.yaml --set image.hash="$(docker inspect --format='{{index .Id}}' ghcr.io/aicacia/auth-api:latest)"`

### Undeploy

- `helm delete -n api auth`
