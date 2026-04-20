# Rust PostgreSQL Starter

Minimal starter project for a new backend service with:

- Rust
- `axum`
- PostgreSQL
- `sqlx` migrations
- Docker and Docker Compose
- GitHub-based remote deployment
- local development helpers via `make`

## API routes

- `GET /`
- `GET /api`
- `GET /health`
- `GET /api/healthchecks`
- `GET /api/healthchecks/:id`
- `POST /api/healthchecks`
- `PUT /api/healthchecks/:id`
- `DELETE /api/healthchecks/:id`

Create or update payload:

```json
{
  "service_name": "system-postgres"
}
```

## Quick start with local PostgreSQL

1. Create an environment file:

```bash
cp .env.example .env
```

2. Initialize and start PostgreSQL:

```bash
make db-init
make db-start
```

3. Apply migrations and run the app:

```bash
make db-migrate
make run
```

## Quick start with Docker Compose

```bash
make compose-up
curl http://127.0.0.1:8080/health
```

Services:

- app: `http://127.0.0.1:8080`
- postgres: `127.0.0.1:5433`
- adminer: `http://127.0.0.1:8081`

## Project layout

- `src/api/`: HTTP routes and handlers
- `src/app.rs`: server startup
- `src/common/`: shared app primitives such as errors
- `src/config.rs`: environment-driven configuration
- `src/db.rs`: PostgreSQL pool and migrations
- `src/domain.rs`: domain entities
- `src/dto.rs`: request and response DTOs
- `src/repos/`: PostgreSQL data access
- `src/services/`: application services and validation
- `migrations/`: SQL schema changes
- `tests/`: integration tests

## Notes

- Embedded migrations are loaded with `sqlx::migrate!`.
- Local PostgreSQL data is stored in `.postgres/data`.
- Docker PostgreSQL data is stored in the named volume `aiinvite_pgdata`.
- Server access and hardening notes are documented in `SERVER_ACCESS.md`.
- GitHub deployment setup is documented in `DEPLOYMENT.md`.
