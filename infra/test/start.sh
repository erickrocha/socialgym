#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
test_cert_dir="${TMPDIR:-/tmp}/socialgym-test-certs"
export TEST_TLS_CERT_DIR="$test_cert_dir"
compose=(docker compose --project-directory "$script_dir" -f "$script_dir/compose.yml")

# Wait for Workout's migrations, then load the fixtures the Timeline acceptance tests expect.
apply_seed() {
  until "${compose[@]}" exec -T postgres psql -U workout_test -d workout_test -tAc \
      "SELECT 1 FROM information_schema.tables WHERE table_name = 'consent'" 2>/dev/null | grep -q 1; do
    sleep 2
  done
  "${compose[@]}" exec -T postgres psql -v ON_ERROR_STOP=1 -U workout_test -d workout_test < "$script_dir/seed.sql"
}

if [[ "${1:-}" == "seed" ]]; then
  apply_seed
  exit 0
fi

if [[ "${1:-}" == "down" ]]; then
  shift
  "${compose[@]}" down "$@"
  rm -f "$test_cert_dir/ca.crt" "$test_cert_dir/ca.key" "$test_cert_dir/ca.srl" \
    "$test_cert_dir/server.crt" "$test_cert_dir/server.csr" "$test_cert_dir/server.key"
  rmdir "$test_cert_dir" 2>/dev/null || true
  exit 0
fi

export PRIVATE_KEY_RAW
PRIVATE_KEY_RAW="$(openssl genrsa -traditional 2048)"
umask 077
mkdir -p "$test_cert_dir"
if [[ ! -s "$test_cert_dir/ca.crt" || ! -s "$test_cert_dir/server.crt" || ! -s "$test_cert_dir/server.key" ]]; then
  rm -f "$test_cert_dir/ca.crt" "$test_cert_dir/ca.key" "$test_cert_dir/ca.srl" \
    "$test_cert_dir/server.crt" "$test_cert_dir/server.csr" "$test_cert_dir/server.key"
  openssl req -x509 -newkey rsa:2048 -nodes \
    -keyout "$test_cert_dir/ca.key" -out "$test_cert_dir/ca.crt" -days 7 \
    -subj '/CN=SocialGym Test CA' \
    -addext 'basicConstraints=critical,CA:TRUE' \
    -addext 'keyUsage=critical,keyCertSign,cRLSign'
  openssl req -newkey rsa:2048 -nodes \
    -keyout "$test_cert_dir/server.key" -out "$test_cert_dir/server.csr" \
    -subj '/CN=integration'
  openssl x509 -req -in "$test_cert_dir/server.csr" \
    -CA "$test_cert_dir/ca.crt" -CAkey "$test_cert_dir/ca.key" -CAcreateserial \
    -out "$test_cert_dir/server.crt" -days 7 \
    -extfile <(printf '%s\n' \
      'basicConstraints=critical,CA:FALSE' \
      'keyUsage=critical,digitalSignature,keyEncipherment' \
      'extendedKeyUsage=serverAuth' \
      'subjectAltName=DNS:integration,DNS:localhost,IP:127.0.0.1')
fi

# The CA certificate is public; the umask above would otherwise make it unreadable by the
# non-root user inside the timeline container (gRPC then fails with "Permission denied").
chmod 644 "$test_cert_dir/ca.crt"

if [[ "${1:-}" == "certs" ]]; then
  printf 'Ephemeral test certificates ready in %s\n' "$test_cert_dir"
  exit 0
fi

"${compose[@]}" up --build -d
apply_seed
