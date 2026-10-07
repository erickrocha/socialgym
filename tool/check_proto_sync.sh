#!/usr/bin/env bash
# Fails when a mirror of a timeline proto differs from its source (SYS-C008-011).
# Source of truth: timeline/business/proto/timeline/. Mirrors: socialgym_mobile/proto/timeline/ for
# every proto except MOBILE_EXCLUDED; workout/integration/proto/timeline/ only for WORKOUT_MIRRORED.
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
src="$root/timeline/business/proto/timeline"
WORKOUT_MIRRORED=(internal.proto)
MOBILE_EXCLUDED=(internal.proto)
status=0
for proto in "$src"/*.proto; do
  name="$(basename "$proto")"
  mirrors=()
  mobile=1
  for x in "${MOBILE_EXCLUDED[@]}"; do [[ "$name" == "$x" ]] && mobile=0; done
  [[ $mobile -eq 1 ]] && mirrors+=("$root/socialgym_mobile/proto/timeline/$name")
  for w in "${WORKOUT_MIRRORED[@]}"; do
    [[ "$name" == "$w" ]] && mirrors+=("$root/workout/integration/proto/timeline/$name")
  done
  for mirror in "${mirrors[@]}"; do
    if ! cmp -s "$proto" "$mirror"; then
      echo "proto drift: ${mirror#"$root"/} differs from ${proto#"$root"/}" >&2
      status=1
    fi
  done
done
[[ $status -eq 0 ]] && echo "proto mirrors in sync"
exit $status
