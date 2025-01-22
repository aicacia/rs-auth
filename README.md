# Aicacia Auth API

[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](LICENSE-MIT)
![Test Status](https://github.com/aicacia/rs-auth/actions/workflows/test.yml/badge.svg)

Aicacia Auth API provides authentication services for applications.

---

## Table of Contents

- [Development Setup](#development-setup)
- [Getting Started](#getting-started)
- [Build Instructions](#build-instructions)
- [Database Migrations](#database-migrations)
- [Docker and Helm](#docker-and-helm)
  - [Deployment](#deployment)
  - [Undeployment](#undeployment)

---

## Development Setup

To set up the development environment:

1. **Install Required Tools:**

   - [rustup](https://rustup.rs/)
   - [cargo-watch](https://crates.io/crates/cargo-watch)
   - [sqlx-cli](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli)

2. **Configure Environment Variables:**

   - Rename the example `.env` file:
     ```bash
     cp .env.example .env
     ```
   - update `DATABASE_URL`

3. **Start Services (Optional only for PostgreSQL):**

   - Create services with Docker Compose:
     ```bash
     docker compose up -d
     ```
   - Delete services when no longer needed:
     ```bash
     docker compose down && docker volume rm auth_postgres
     ```

4. **Create Database:**

   - Create database
     ```bash
     sqlx database create
     ```
   - Run Migrations
     ```bash
     sqlx migrate run --source ./migrations/${database}/
     ```

5. **Run the Main Web Service:**

   - Use cargo-watch to start the service:
     ```bash
     cargo watch -c -w src -x run
     ```

6. **View API Documentation:**
   - Access the OpenAPI Docs:  
     [OpenAPI Documentation](https://petstore.swagger.io/?url=http://localhost:3000/openapi.json)

---

## Getting Started

On startup, if no service accounts exist, the service creates a new one and saves it to the current working directory as `auth-admin-service-account.json`.

### Default Tenant ID

The following default Tenant ID is provided for convenience:

```plaintext
Tenant-ID: 6fcf0235-cb11-4160-9df8-b9114f8dcdae
```

### Creating a Service Account Token

Use the following JSON structure to create a service account token:

```json
{
  "grant_type": "service-account",
  "client_id": "...",
  "secret": "..."
}
```

---

## Build Instructions

To build the project locally:

```bash
cargo install --path .
```

### Extra Commands

- Drop the database:

  ```bash
  sqlx database drop
  ```

- Revert the last migration:
  ```bash
  sqlx migrate revert --source ./migrations/${database}/
  ```
- Prepare for offline usage:
  ```bash
  cargo sqlx prepare
  ```

---

## Docker and Helm

### Deployment

To build and deploy the service using Docker and Helm:

1. **Build the Docker image:**

   ```bash
   # build for x86_64
   docker build -t aicacia/auth-api:0.1-x86_64 .

   # build for armv7
   cross build --target armv7-unknown-linux-musleabihf --release
   docker buildx build --no-cache -o type=docker --push --platform linux/arm/v7 --build-arg=TARGET=armv7-unknown-linux-musleabihf -t aicacia/auth-api:0.1-armv7 -f Dockerfile.local-target .
   ```

2. **Push the image to the registry:**

   ```bash
   docker push aicacia/auth-api:0.1-x86_64
   ```

3. **Deploy with Helm:**

   ```bash
   helm upgrade auth-api helm/auth-api -n api --install -f values.yaml --set image.hash="$(docker inspect --format='{{index .Id}}' aicacia/auth-api:0.1-x86_64)"
   ```

4. **Deploy locally**
   ```bash
   docker run -it \
    -p 3000:3000 \
    -v ${PWD}/.env:/app/.env \
    -v ${PWD}/config.json:/app/config.json \
    -v ${PWD}/auth-dev.db:/app/auth-dev.db \
    aicacia/auth-api:0.1-x86_64
   ```

### Undeployment

To undeploy the service:

```bash
helm delete -n api auth-api
```

## OpenAPI Client

```bash
rm -rf auth-client && \
openapi-generator-cli generate -i ./openapi.json -g rust -o 'auth-client' --additional-properties=packageName=auth-client,library=hyper,bestFitInt=true,avoidBoxedModels=true
```
