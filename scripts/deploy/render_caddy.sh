#!/usr/bin/env bash
set -euo pipefail

: "${CADDY_SITE_ADDRESS:?CADDY_SITE_ADDRESS must be set}"

cat > deploy/Caddyfile <<EOF
${CADDY_SITE_ADDRESS} {
    encode zstd gzip

    reverse_proxy app:8080
}
EOF
