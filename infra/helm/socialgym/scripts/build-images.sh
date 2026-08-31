#!/usr/bin/env bash
# Builds workout-app, integration-app, and timeline-app directly into
# minikube's Docker daemon, so the chart's `imagePullPolicy: IfNotPresent`
# can use them with no registry involved.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"

eval "$(minikube docker-env)"

docker build -t workout-app:1.0.0 -f "$REPO_ROOT/workout/Dockerfile" "$REPO_ROOT/workout"
docker build -t integration-app:1.0.0 -f "$REPO_ROOT/workout/integration/Dockerfile" "$REPO_ROOT/workout"
docker build -t timeline-app:1.0.0 -f "$REPO_ROOT/timeline/Dockerfile" "$REPO_ROOT/timeline"

echo "Images built into minikube's docker daemon: workout-app:local, integration-app:local, timeline-app:local"
