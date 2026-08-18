#!/usr/bin/env bash
# Brings up the whole SocialGym backend on minikube: starts the cluster if
# needed, generates/loads the dev TLS cert, builds the three Rust service
# images into minikube's docker daemon, and installs/upgrades the Helm
# release, waiting for every Deployment to become ready.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CHART_DIR="$(dirname "$SCRIPT_DIR")"
NAMESPACE="${SOCIALGYM_NAMESPACE:-socialgym-dev}"
RELEASE="${SOCIALGYM_RELEASE:-socialgym}"

if ! minikube status >/dev/null 2>&1; then
  echo "Starting minikube..."
  minikube start --driver=docker --cpus=4 --memory=6g
fi

"$SCRIPT_DIR/gen-dev-certs.sh"
"$SCRIPT_DIR/build-images.sh"

helm upgrade --install "$RELEASE" "$CHART_DIR" -n "$NAMESPACE" --create-namespace "$@"

for deploy in postgres mongo workout integration timeline gateway; do
  kubectl rollout status "deployment/$deploy" -n "$NAMESPACE" --timeout=180s
done

echo
echo "Backend is up. Reach it with:"
echo "  minikube service gateway -n $NAMESPACE --url"
