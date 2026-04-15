# AIInvite Rust Environment

This project now includes:

- a multi-page marketing website for an event decor studio
- an `axum` HTTP API
- embedded `sqlx` migrations
- a local PostgreSQL workflow in the project directory
- a Docker Compose workflow for app + PostgreSQL + Adminer

## Website routes

- `GET /`
- `GET /about`
- `GET /services`
- `GET /services/:slug`
- `GET /portfolio`
- `GET /portfolio/:slug`
- `GET /prices`
- `GET /reviews`
- `GET /contacts`
- `GET /blog`
- `GET /blog/:slug`
- `POST /request`

The website ships with:

- a premium multi-page layout for an event decoration brand
- service landing pages for SEO
- portfolio case pages
- reviews, pricing, contacts, and blog pages
- repeated lead capture forms
- PostgreSQL persistence for submitted lead requests

## API routes

- `GET /api`
- `GET /`
- `GET /health`
- `GET /api/healthchecks`
- `GET /api/healthchecks/:id`
- `POST /api/healthchecks`
- `PUT /api/healthchecks/:id`
- `DELETE /api/healthchecks/:id`

Example payload:

```json
{
  "service_name": "system-postgres"
}
```

Update payload:

```json
{
  "service_name": "renamed-service"
}
```

## Quick start with system PostgreSQL

1. Initialize the local cluster:

```bash
make db-init
make db-start
```

2. Ensure `.env` exists:

```bash
cp .env.example .env
```

3. Apply migrations and run the API:

```bash
make db-migrate
make run
```

## Quick start with Docker Compose

```bash
make compose-up
curl http://127.0.0.1:8080/health
```

Compose services:

- app: `http://127.0.0.1:8080`
- postgres: `127.0.0.1:5433`
- adminer: `http://127.0.0.1:8081`

`make compose-up` first builds a local release binary and then packages that binary into the app container.

## Notes

- Embedded migrations are loaded from `migrations/` via `sqlx::migrate!`.
- Lead form submissions are stored in the `lead_requests` table.
- Runtime modules are split across `src/app.rs`, `src/config.rs`, `src/db.rs`, `src/error.rs`, `src/handlers.rs`, and `src/models.rs`.
- Local project PostgreSQL data is stored in `.postgres/data`.
- Docker PostgreSQL data is stored in the named volume `aiinvite_pgdata`.
- If `docker` requires `sudo`, start a new shell session so group membership is refreshed.
