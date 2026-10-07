#!/usr/bin/env bash
# Fails when a `workout` REST route has no row in workout/rest_inventory.csv, when a row names a route
# that no longer exists, or when a row is still marked `verify` (C-010 SYS-C010-001).
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
inv="$root/workout/rest_inventory.csv"
status=0
routes="$(python3 "$root/tool/rest_routes.py" | sort)"
rows="$(python3 - "$inv" <<'PY' | sort
import csv, sys
for r in csv.DictReader(open(sys.argv[1])):
    print(r["route"])
PY
)"
missing="$(comm -23 <(echo "$routes") <(echo "$rows"))"
stale="$(comm -13 <(echo "$routes") <(echo "$rows"))"
[[ -z "$missing" ]] || { echo "REST routes without a row in $inv:"; echo "$missing"; status=1; }
[[ -z "$stale" ]] || { echo "rows for routes that no longer exist:"; echo "$stale"; status=1; }
if python3 - "$inv" <<'PY'
import csv, sys
bad = [r["route"] for r in csv.DictReader(open(sys.argv[1])) if r["status"] not in ("exists", "needs", "REST")]
if bad:
    print("rows with an undecided status (use exists, needs or REST):"); print("\n".join(bad)); sys.exit(1)
PY
then :; else status=1; fi
[[ $status -ne 0 ]] || echo "REST inventory complete: $(echo "$routes" | wc -l) routes"
exit $status
