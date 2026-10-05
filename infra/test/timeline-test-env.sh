#!/usr/bin/env bash
# Environment for running the Timeline acceptance tests (`cargo test -- --include-ignored`)
# from the host against the infra/test stack. Source it:  source infra/test/timeline-test-env.sh
# Requires `./start.sh` to be up (it also loads seed.sql, which several tests rely on).

certs="${TEST_TLS_CERT_DIR:-/tmp/socialgym-test-certs}"

export TEST_MONGO_URL="mongodb://timeline_test:timeline_test@localhost:37017/timeline_test?authSource=timeline_test"

source "$(dirname "${BASH_SOURCE[0]}")/aws-test-env.sh"

export GRPC_PROTOCOL="https"
export GRPC_HOST="localhost"
export GRPC_PORT="18501"
export GRPC_USE_TLS="true"
export GRPC_DOMAIN_NAME="integration"
export GRPC_CERT_PATH="$certs/ca.crt"

export INTERNAL_SERVICE_SECRET="c005-internal-secret"
export ACCESS_TOKEN_SECRET="c005-test-secret"
