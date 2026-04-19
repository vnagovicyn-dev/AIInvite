#!/usr/bin/env bash
set -euo pipefail

release_dir="${1:-$(pwd)}"
current_link="${2:-/srv/aiinvite/current}"
project_name="${PROJECT_NAME:-aiinvite}"

if [[ ! -d "$release_dir" ]]; then
  echo "release directory not found: $release_dir" >&2
  exit 1
fi

if [[ ! -f "$release_dir/.env.production" ]]; then
  echo "missing production env file in $release_dir" >&2
  exit 1
fi

if [[ ! -f "$release_dir/deploy/Caddyfile" ]]; then
  echo "missing rendered Caddyfile in $release_dir/deploy/Caddyfile" >&2
  exit 1
fi

mkdir -p "$(dirname "$current_link")"
ln -sfn "$release_dir" "$current_link"

compose_args=(
  -p "$project_name"
  -f "$current_link/docker-compose.production.yml"
  --env-file "$current_link/.env.production"
)

docker compose "${compose_args[@]}" pull caddy redis
docker compose "${compose_args[@]}" up -d --build --remove-orphans
docker image prune -f >/dev/null 2>&1 || true

if [[ -d /srv/aiinvite/releases ]]; then
  ls -1dt /srv/aiinvite/releases/* 2>/dev/null | tail -n +6 | xargs -r rm -rf
fi
