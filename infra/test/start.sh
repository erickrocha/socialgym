#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
export PRIVATE_KEY_RAW
PRIVATE_KEY_RAW="$(openssl genrsa -traditional 2048)"

docker compose \
  --project-directory "$script_dir" \
  -f "$script_dir/compose.yml" \
  up --build -d
