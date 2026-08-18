#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PROTO_DIR="$ROOT_DIR/proto"
OUT_DIR="$ROOT_DIR/lib/src/generated/grpc"

if ! command -v protoc >/dev/null 2>&1; then
  echo "protoc not found. Install protobuf compiler first (e.g. sudo apt install protobuf-compiler)." >&2
  exit 1
fi

mkdir -p "$OUT_DIR"

protoc \
  --proto_path="$PROTO_DIR" \
  --dart_out=grpc:"$OUT_DIR" \
  "$PROTO_DIR"/*.proto

echo "Generated Dart gRPC files in $OUT_DIR"

