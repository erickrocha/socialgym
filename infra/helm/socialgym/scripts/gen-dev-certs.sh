#!/usr/bin/env bash
# Generates a fresh self-signed TLS cert for local minikube use (gateway +
# integration's gRPC listener) and loads it into the cluster as the
# `socialgym-tls` Secret. Never reuses infra/certs/ or infra/dev/certs/ -
# those are separate, already-committed dev certs; this one is generated
# fresh per machine and is gitignored.
set -euo pipefail

NAMESPACE="${SOCIALGYM_NAMESPACE:-socialgym-dev}"
SECRET_NAME="${SOCIALGYM_TLS_SECRET:-socialgym-tls}"
CERT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)/.dev-certs"

mkdir -p "$CERT_DIR"

if [[ ! -f "$CERT_DIR/server.crt" || ! -f "$CERT_DIR/server.key" ]]; then
  echo "Generating a fresh self-signed dev cert in $CERT_DIR ..."
  openssl req -x509 -nodes -newkey rsa:2048 -days 825 \
    -keyout "$CERT_DIR/server.key" \
    -out "$CERT_DIR/server.crt" \
    -subj "/CN=socialgym.local" \
    -addext "subjectAltName=DNS:socialgym.local,DNS:gateway,DNS:integration,DNS:localhost,IP:127.0.0.1"
else
  echo "Reusing existing dev cert in $CERT_DIR"
fi

kubectl create namespace "$NAMESPACE" --dry-run=client -o yaml | kubectl apply -f -
kubectl create secret tls "$SECRET_NAME" \
  --cert="$CERT_DIR/server.crt" \
  --key="$CERT_DIR/server.key" \
  -n "$NAMESPACE" \
  --dry-run=client -o yaml | kubectl apply -f -

echo "TLS secret '$SECRET_NAME' ready in namespace '$NAMESPACE'."
