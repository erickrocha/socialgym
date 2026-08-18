#!/usr/bin/env bash
# Tears down the Helm release. Leaves the minikube cluster running - pass
# `--delete-cluster` to also stop it, or run `minikube delete` yourself for
# a full wipe (including the postgres/mongo PVCs).
set -euo pipefail

NAMESPACE="${SOCIALGYM_NAMESPACE:-socialgym-dev}"
RELEASE="${SOCIALGYM_RELEASE:-socialgym}"

helm uninstall "$RELEASE" -n "$NAMESPACE" || true

if [[ "${1:-}" == "--delete-cluster" ]]; then
  minikube stop
fi
