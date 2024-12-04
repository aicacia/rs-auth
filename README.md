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
- View [OpenApi Docs](https://petstore.swagger.io/?url=http://localhost:3000/openapi.json)
- create services `docker compose up -d`
- delete services `docker compose down` and `docker volume rm auth_postgres`

## Default token request for testing only should be deleted

```
Tenent-ID: 6fcf0235-cb11-4160-9df8-b9114f8dcdae
```

```json
{
  "grant_type": "password",
  "scope": "openid",
  "password": "password",
  "username": "test"
}
```

```json
{
  "grant_type": "service-account",
  "client_id": "dba9fb13-f2d0-498e-aaf2-65c435ffe797",
  "secret": "7694ab3c-a1e0-4345-accc-85504ad475d8"
}
```

## Build

- `cargo install --path .`

## Migrations

- create the database `sqlx database create`
- run migrations `sqlx migrate run --source ./migrations/${database-type}/`
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
