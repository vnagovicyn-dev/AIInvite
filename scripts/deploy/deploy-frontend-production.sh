#!/usr/bin/env bash
set -euo pipefail

: "${FRONTEND_DIR:=/srv/aiinvite/current/frontend}"
: "${SERVICE_NAME:=aiinvite-frontend}"
: "${PORT:=3000}"
: "${HOST:=0.0.0.0}"
: "${NODE_ENV:=production}"
: "${API_BASE_URL_INTERNAL:=http://127.0.0.1:8080}"
: "${NEXT_PUBLIC_API_BASE_URL:=http://127.0.0.1:8080}"
: "${SERVICE_USER:=$(id -un)}"

run_sudo() {
  if sudo -n true >/dev/null 2>&1; then
    sudo -n "$@"
  else
    sudo "$@"
  fi
}

if [[ ! -d "$FRONTEND_DIR" ]]; then
  echo "frontend directory not found: $FRONTEND_DIR" >&2
  exit 1
fi

cd "$FRONTEND_DIR"

if [[ ! -f package.json ]]; then
  echo "package.json not found in $FRONTEND_DIR" >&2
  exit 1
fi

if [[ ! -f .env.local ]]; then
  cat > .env.local <<EOF
NEXT_PUBLIC_API_BASE_URL=${NEXT_PUBLIC_API_BASE_URL}
API_BASE_URL_INTERNAL=${API_BASE_URL_INTERNAL}
EOF
else
  if grep -q '^NEXT_PUBLIC_API_BASE_URL=' .env.local; then
    sed -i "s#^NEXT_PUBLIC_API_BASE_URL=.*#NEXT_PUBLIC_API_BASE_URL=${NEXT_PUBLIC_API_BASE_URL}#" .env.local
  else
    printf 'NEXT_PUBLIC_API_BASE_URL=%s\n' "${NEXT_PUBLIC_API_BASE_URL}" >> .env.local
  fi

  if grep -q '^API_BASE_URL_INTERNAL=' .env.local; then
    sed -i "s#^API_BASE_URL_INTERNAL=.*#API_BASE_URL_INTERNAL=${API_BASE_URL_INTERNAL}#" .env.local
  else
    printf 'API_BASE_URL_INTERNAL=%s\n' "${API_BASE_URL_INTERNAL}" >> .env.local
  fi
fi

if [[ -f package-lock.json ]]; then
  npm ci
else
  npm install
fi

rm -rf .next
npm run build

cat > "/tmp/${SERVICE_NAME}.service" <<EOF
[Unit]
Description=AIInvite Frontend
After=network.target

[Service]
Type=simple
User=${SERVICE_USER}
WorkingDirectory=${FRONTEND_DIR}
Environment=NODE_ENV=${NODE_ENV}
ExecStart=/usr/bin/npm run start -- --hostname ${HOST} --port ${PORT}
Restart=always
RestartSec=3

[Install]
WantedBy=multi-user.target
EOF

run_sudo install -D -m 0644 "/tmp/${SERVICE_NAME}.service" "/etc/systemd/system/${SERVICE_NAME}.service"
run_sudo systemctl daemon-reload
run_sudo systemctl enable --now "${SERVICE_NAME}"
run_sudo systemctl restart "${SERVICE_NAME}"
run_sudo systemctl --no-pager --full status "${SERVICE_NAME}" | sed -n '1,20p'
