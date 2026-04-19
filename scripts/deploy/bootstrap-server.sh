#!/usr/bin/env bash
set -euo pipefail

: "${APP_DB_NAME:=aiinvite_production}"
: "${APP_DB_USER:=aiinvite}"
: "${APP_DB_PASSWORD:?APP_DB_PASSWORD must be set}"
: "${DEPLOY_USER:=deploy}"
: "${DEPLOY_PUBLIC_KEY:?DEPLOY_PUBLIC_KEY must be set}"
: "${DEPLOY_BASE:=/srv/aiinvite}"

export DEBIAN_FRONTEND=noninteractive

apt-get update
if apt-cache show docker-compose-v2 >/dev/null 2>&1; then
  apt-get install -y docker.io docker-compose-v2 postgresql postgresql-contrib
else
  apt-get install -y docker.io docker-compose-plugin postgresql postgresql-contrib
fi

systemctl enable --now docker
systemctl enable --now postgresql

if ! id -u "$DEPLOY_USER" >/dev/null 2>&1; then
  adduser --disabled-password --gecos "" "$DEPLOY_USER"
fi

usermod -aG docker "$DEPLOY_USER"

install -d -m 700 -o "$DEPLOY_USER" -g "$DEPLOY_USER" "/home/$DEPLOY_USER/.ssh"
install -m 600 -o "$DEPLOY_USER" -g "$DEPLOY_USER" /dev/null "/home/$DEPLOY_USER/.ssh/authorized_keys"
grep -qxF "$DEPLOY_PUBLIC_KEY" "/home/$DEPLOY_USER/.ssh/authorized_keys" || printf '%s\n' "$DEPLOY_PUBLIC_KEY" >> "/home/$DEPLOY_USER/.ssh/authorized_keys"

install -d -m 755 -o "$DEPLOY_USER" -g "$DEPLOY_USER" "$DEPLOY_BASE"
install -d -m 755 -o "$DEPLOY_USER" -g "$DEPLOY_USER" "$DEPLOY_BASE/releases"

sudo -u postgres psql -v ON_ERROR_STOP=1 <<SQL
DO \$\$
BEGIN
    IF NOT EXISTS (SELECT FROM pg_catalog.pg_roles WHERE rolname = '${APP_DB_USER}') THEN
        CREATE ROLE ${APP_DB_USER} LOGIN PASSWORD '${APP_DB_PASSWORD}';
    ELSE
        ALTER ROLE ${APP_DB_USER} WITH LOGIN PASSWORD '${APP_DB_PASSWORD}';
    END IF;
END
\$\$;
SQL

sudo -u postgres psql -tc "SELECT 1 FROM pg_database WHERE datname = '${APP_DB_NAME}'" | grep -q 1 || sudo -u postgres createdb -O "${APP_DB_USER}" "${APP_DB_NAME}"

postgres_version="$(sudo -u postgres psql -Atqc 'SHOW server_version_num' | cut -c1-2)"
postgres_dir="/etc/postgresql/${postgres_version}/main"
postgresql_conf="${postgres_dir}/postgresql.conf"
pg_hba_conf="${postgres_dir}/pg_hba.conf"

sed -i "s/^#\?listen_addresses.*/listen_addresses = '*'/" "$postgresql_conf"
grep -qxF "host    ${APP_DB_NAME}    ${APP_DB_USER}    172.16.0.0/12    scram-sha-256" "$pg_hba_conf" || printf '\nhost    %s    %s    172.16.0.0/12    scram-sha-256\n' "$APP_DB_NAME" "$APP_DB_USER" >> "$pg_hba_conf"

systemctl restart postgresql

ufw allow 80/tcp
ufw allow 443/tcp
ufw allow from 172.16.0.0/12 to any port 5432 proto tcp
