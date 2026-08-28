#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
legal_dir="$repo_root/workout/application/resources/legal/pt-BR"
env_file="${1:-$repo_root/infra/prod/.env}"

if rg -n -i 'minuta|sujeit[oa] à aprovação|devem ser inseridos|PENDENTE|CHANGE_ME' "$legal_dir"; then
  echo "Compliance gate failed: legal documents still contain release blockers." >&2
  exit 1
fi

if [[ ! -f "$env_file" ]]; then
  echo "Compliance gate failed: production environment file not found: $env_file" >&2
  exit 1
fi

required=(
  WORKOUT_AWS_REGION TIMELINE_AWS_REGION INTERNAL_SERVICE_SECRET
  TERMS_VERSION PRIVACY_VERSION HEALTH_DATA_CONSENT_VERSION
)
for key in "${required[@]}"; do
  value="$(awk -F= -v key="$key" '$1 == key {sub(/^[^=]*=/, ""); print; exit}' "$env_file")"
  if [[ -z "$value" || "$value" == "CHANGE_ME" ]]; then
    echo "Compliance gate failed: $key is absent or unresolved in $env_file." >&2
    exit 1
  fi
done

echo "Automated compliance gate passed. Attach legal, DPO, retention, transfer and security checklist approvals to the release."
