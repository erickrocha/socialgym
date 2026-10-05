#!/usr/bin/env bash
# Line coverage for a Rust service, including the #[ignore]d database acceptance tests.
# Usage: ./coverage.sh timeline|workout
# Both need the infra/test stack up (./start.sh): timeline uses it fully; workout uses its LocalStack
# SQS but starts its own PostGIS so the stack's database is not refreshed.
# Set COVERAGE_IGNORE_RUN_FAIL=1 to still report when some tests fail.
set -euo pipefail

service="${1:?usage: coverage.sh timeline|workout}"
root="$(cd "$(dirname "$0")/../.." && pwd)"
container="sg-cov-$service"

# Measurement scope (decision 2026-10-05): exclude only entrypoints, route builders, bootstrap
# wiring, JSON DTOs, ORM entities, migrations and generated code. Mappers and I/O adapters stay in.
exclude='(^|/)src/main\.rs$|/routes/|application/src/lib\.rs$|/json/|(^|/)entity/|(^|/)migration/|/generated/|\.pb\.rs$'

trap 'docker rm -f "$container" >/dev/null 2>&1 || true' EXIT

case "$service" in
  timeline)
    # Needs the full stack (Mongo, LocalStack SQS, Workout gRPC + seed): run ./start.sh first.
    (exec 3<>/dev/tcp/localhost/37017) 2>/dev/null || { echo "infra/test stack is not up; run ./start.sh" >&2; exit 1; }
    # shellcheck source=timeline-test-env.sh
    source "$(dirname "$0")/timeline-test-env.sh"
    ;;
  workout)
    docker run -d --rm --name "$container" -e POSTGRES_DB=workout_test -e POSTGRES_USER=workout_test \
      -e POSTGRES_PASSWORD=workout_test -p 55499:5432 postgis/postgis:18-3.6 >/dev/null
    until docker exec "$container" pg_isready -U workout_test -d workout_test >/dev/null 2>&1; do sleep 2; done
    export TEST_DATABASE_URL="postgres://workout_test:workout_test@localhost:55499/workout_test"
    # The friendship outbox test publishes to SQS: reuse the stack's LocalStack (database stays isolated).
    (exec 3<>/dev/tcp/localhost/4566) 2>/dev/null || { echo "infra/test stack is not up (LocalStack); run ./start.sh" >&2; exit 1; }
    source "$(dirname "$0")/aws-test-env.sh"
    ;;
  *) echo "unknown service: $service" >&2; exit 2 ;;
esac

cd "$root/$service"
extra=()
[ "${COVERAGE_IGNORE_RUN_FAIL:-0}" = 1 ] && extra+=(--ignore-run-fail)

# ponytail: --test-threads=1 because acceptance tests share one disposable database.
cargo llvm-cov --workspace --all-features "${extra[@]}" --summary-only \
  --ignore-filename-regex "$exclude" -- --include-ignored --test-threads=1
