#!/usr/bin/env bash
# The nginx gateway of the infra/test stack, for clients that talk to one origin (the mobile app):
# the same routes as infra/dev, pointed at the stack's containers. REST (cleartext) answers on 8080,
# TLS (REST and gRPC) on 8443, with the stack's test certificate, which names 10.0.2.2 (the host
# as the Android emulator sees it). Needs `start.sh` first.
#   ./e2e-gateway.sh up | down | log
set -euo pipefail

script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
dev_dir="$script_dir/../dev"
cert_dir="${TEST_TLS_CERT_DIR:-${TMPDIR:-/tmp}/socialgym-test-certs}"
conf_dir="${TMPDIR:-/tmp}/socialgym-test-gateway"
name=test-gateway

case "${1:-up}" in
  up)
    mkdir -p "$conf_dir"
    # Same files as the dev gateway; only the upstream addresses change (the stack's service names).
    sed -e 's#host.docker.internal:8090#workout:8090#g' \
        -e 's#host.docker.internal:8091#timeline:8091#g' \
        -e 's#host.docker.internal:50051#integration:50051#g' \
        -e 's#host.docker.internal:50052#timeline:50052#g' \
        "$dev_dir/nginx.conf" > "$conf_dir/nginx.conf"
    cp "$dev_dir/locations.conf" "$conf_dir/locations.conf"
    docker rm -f "$name" >/dev/null 2>&1 || true
    docker run -d --name "$name" --network socialgym-test -p 8080:8080 -p 8443:443 \
      -v "$conf_dir/nginx.conf:/etc/nginx/nginx.conf:ro" \
      -v "$conf_dir/locations.conf:/etc/nginx/locations.conf:ro" \
      -v "$cert_dir/server.crt:/etc/nginx/certs/server.crt:ro" \
      -v "$cert_dir/server.key:/etc/nginx/certs/server.key:ro" \
      nginx:alpine >/dev/null
    sleep 2
    docker logs "$name" 2>&1 | tail -3
    ;;
  down)
    docker rm -f "$name" >/dev/null 2>&1 || true
    rm -rf "$conf_dir"
    ;;
  log)
    docker logs "$name" 2>&1
    ;;
  *) echo "usage: $0 up|down|log" >&2; exit 2 ;;
esac
