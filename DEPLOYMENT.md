# GitHub Auto-Deploy

This project is configured for automatic deployment to a remote Linux server on every push to `main`.

Compatibility note:

- The workflow also listens to `master` because the local repository is currently on that branch.
- If you rename the default branch to `main`, deployment will continue to work without changes.

## Deployment Topology

- GitHub Actions uploads the current repository snapshot to the server
- The server keeps releases in `/srv/aiinvite/releases/<git-sha>`
- `/srv/aiinvite/current` points to the active release
- Docker Compose runs:
  - Rust application
  - Caddy
  - Redis
- PostgreSQL runs directly on the server, outside Docker

## Production Files

- Workflow: `.github/workflows/deploy.yml`
- Production Compose: `docker-compose.production.yml`
- Deploy script: `scripts/deploy/remote-deploy.sh`
- Server bootstrap script: `scripts/deploy/bootstrap-server.sh`
- Caddy template: `deploy/Caddyfile.template`
- Production env example: `deploy/.env.production.example`

## GitHub Secrets To Add

Add these secrets in the private GitHub repository before the workflow can deploy:

- `DEPLOY_HOST`
  - Example: `159.194.216.45`
- `DEPLOY_USER`
  - Example: `deploy`
- `DEPLOY_SSH_KEY`
  - The private key for the dedicated deployment SSH key
- `APP_DATABASE_URL`
  - Example format: `postgresql://aiinvite:<url-safe-password>@host.docker.internal:5432/aiinvite_production`
  - Prefer a URL-safe password with only letters and digits unless you are explicitly URL-encoding special characters
- `REDIS_PASSWORD`
  - Strong password used by the Redis container
- `CADDY_SITE_ADDRESS`
  - Domain example: `api.example.com`
  - IP-only fallback: `http://159.194.216.45`

## First-Time Server Bootstrap

Server bootstrap is handled by:

- `scripts/deploy/bootstrap-server.sh`

It installs:

- Docker Engine
- Docker Compose v2
- PostgreSQL

And it configures:

- deploy user for GitHub Actions
- `/srv/aiinvite` release directories
- PostgreSQL database and role
- PostgreSQL access for Docker containers on the host bridge
- Docker subnet access in PostgreSQL and UFW via `172.16.0.0/12`
- UFW rules for `80/tcp` and `443/tcp`

## Deployment Flow

1. Push to `main`
2. GitHub Actions renders `.env.production` and `deploy/Caddyfile`
3. GitHub Actions uploads the new release to `/srv/aiinvite/releases/<sha>`
4. The server rebuilds the Rust image and updates the stack via Docker Compose
5. Old releases are pruned automatically, keeping the most recent ones
