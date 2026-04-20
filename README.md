# Invite Platform Backend

Minimal backend scaffold for an invitation service built with Rust, `axum`, `sqlx`, PostgreSQL and `utoipa`.

## Required environment variables

- `APP_NAME`
- `APP_HOST`
- `APP_PORT`
- `RUST_LOG`
- `DATABASE_URL`

## Local start

1. Create env file:

```bash
cp .env.example .env
```

2. Start PostgreSQL. Example with Docker:

```bash
docker run --name invite-platform-postgres \
  -e POSTGRES_PASSWORD=postgres \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_DB=invite_platform \
  -p 5432:5432 \
  -d postgres:16
```

3. Run the application:

```bash
cargo run
```

On startup the app will:
- create a PostgreSQL connection pool
- apply embedded SQL migrations from `migrations/`
- start HTTP server on the configured host and port

## Routes

- `GET /api/health`
- `GET /api/docs`
- `GET /api/openapi.json`

## Migrations

- Migrations are embedded with `sqlx::migrate!()`.
- Current bootstrap migration: `migrations/0001_init_core.sql`
