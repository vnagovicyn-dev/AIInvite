#!/usr/bin/env bash
set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PGROOT="${PROJECT_ROOT}/.postgres"
PGDATA="${PGROOT}/data"
PGSOCKET="${PGROOT}"
PGLOG="${PGROOT}/postgres.log"
PGPORT="${PGPORT:-5432}"
PGUSER="${PGUSER:-aiinvite}"
PGPASSWORD="${PGPASSWORD:-aiinvite}"
PGDATABASE="${PGDATABASE:-aiinvite_dev}"

export PGHOST="${PGSOCKET}"
export PGDATA
export PGPORT
export PGUSER
export PGPASSWORD
export PGDATABASE
export DATABASE_URL="postgresql://${PGUSER}:${PGPASSWORD}@127.0.0.1:${PGPORT}/${PGDATABASE}"

ensure_tools() {
  command -v initdb >/dev/null 2>&1 || {
    echo "initdb not found. Enter the dev shell first: nix develop"
    exit 1
  }
  command -v pg_ctl >/dev/null 2>&1 || {
    echo "pg_ctl not found. Enter the dev shell first: nix develop"
    exit 1
  }
  command -v psql >/dev/null 2>&1 || {
    echo "psql not found. Enter the dev shell first: nix develop"
    exit 1
  }
}

init_db() {
  ensure_tools
  mkdir -p "${PGROOT}"

  if [ ! -d "${PGDATA}" ]; then
    initdb --username="${PGUSER}" --auth=trust --pgdata="${PGDATA}" >/dev/null
  fi

  if ! grep -q "^unix_socket_directories = '${PGSOCKET}'" "${PGDATA}/postgresql.conf"; then
    cat >>"${PGDATA}/postgresql.conf" <<EOF
listen_addresses = '127.0.0.1'
port = ${PGPORT}
unix_socket_directories = '${PGSOCKET}'
EOF
  fi

  pg_ctl -D "${PGDATA}" -l "${PGLOG}" start >/dev/null
  createdb --if-not-exists "${PGDATABASE}" >/dev/null 2>&1 || true
  psql -d postgres -v ON_ERROR_STOP=1 <<EOF >/dev/null
DO \$\$
BEGIN
  IF NOT EXISTS (SELECT FROM pg_catalog.pg_roles WHERE rolname = '${PGUSER}') THEN
    CREATE ROLE ${PGUSER} LOGIN SUPERUSER PASSWORD '${PGPASSWORD}';
  END IF;
END
\$\$;
EOF
  pg_ctl -D "${PGDATA}" stop >/dev/null

  echo "PostgreSQL cluster initialized in ${PGDATA}"
}

start_db() {
  ensure_tools
  [ -d "${PGDATA}" ] || init_db
  if pg_ctl -D "${PGDATA}" status >/dev/null 2>&1; then
    echo "PostgreSQL is already running"
    return 0
  fi
  pg_ctl -D "${PGDATA}" -l "${PGLOG}" start >/dev/null
  echo "PostgreSQL started on port ${PGPORT}"
}

stop_db() {
  ensure_tools
  if [ -d "${PGDATA}" ] && pg_ctl -D "${PGDATA}" status >/dev/null 2>&1; then
    pg_ctl -D "${PGDATA}" stop >/dev/null
    echo "PostgreSQL stopped"
  else
    echo "PostgreSQL is not running"
  fi
}

status_db() {
  ensure_tools
  if [ -d "${PGDATA}" ] && pg_ctl -D "${PGDATA}" status >/dev/null 2>&1; then
    pg_ctl -D "${PGDATA}" status
  else
    echo "PostgreSQL is not running"
  fi
}

reset_db() {
  ensure_tools
  if [ -d "${PGDATA}" ] && pg_ctl -D "${PGDATA}" status >/dev/null 2>&1; then
    pg_ctl -D "${PGDATA}" stop >/dev/null
  fi
  rm -rf "${PGDATA}" "${PGLOG}"
  init_db
  echo "PostgreSQL cluster reset"
}

migrate_db() {
  ensure_tools
  command -v cargo >/dev/null 2>&1 || {
    echo "cargo not found."
    exit 1
  }

  if ! [ -d "${PGDATA}" ] || ! pg_ctl -D "${PGDATA}" status >/dev/null 2>&1; then
    echo "PostgreSQL must be running before migrations"
    exit 1
  fi

  (
    cd "${PROJECT_ROOT}"
    cargo run -- migrate
  )
}

case "${1:-}" in
  init)
    init_db
    ;;
  start)
    start_db
    ;;
  stop)
    stop_db
    ;;
  status)
    status_db
    ;;
  reset)
    reset_db
    ;;
  migrate)
    migrate_db
    ;;
  *)
    echo "Usage: $0 {init|start|stop|status|reset|migrate}"
    exit 1
    ;;
esac
