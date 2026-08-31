#!/usr/bin/env bash
# Bring the SocialGym backend up on minikube (via the Helm chart's up.sh),
# then expose the gateway's fixed HTTPS NodePort through ngrok.
#
# Any arguments are forwarded to infra/helm/socialgym/scripts/up.sh (and on to
# `helm upgrade --install`). up.sh runs from the chart dir, so a `-f` path is
# resolved relative to infra/helm/socialgym/, e.g.:
#   ./ngrok-up.sh -f values.dev-aws.yaml
#
# Env overrides:
#   NGROK_DOMAIN            reserved ngrok domain (this is the one already in
#                           ~/.config/ngrok/ngrok.yml) [scrutiny-elevator-washstand.ngrok-free.dev]
#   GATEWAY_HTTPS_NODEPORT  NodePort to tunnel to  [30443, matches values.yaml gateway.httpsNodePort]
#   SOCIALGYM_NAMESPACE     k8s namespace          [socialgym-dev]
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CHART_DIR="$REPO_ROOT/infra/helm/socialgym"
UP_SH="$CHART_DIR/scripts/up.sh"

NGROK_DOMAIN="${NGROK_DOMAIN:-scrutiny-elevator-washstand.ngrok-free.dev}"
GATEWAY_HTTPS_NODEPORT="${GATEWAY_HTTPS_NODEPORT:-30443}"
NAMESPACE="${SOCIALGYM_NAMESPACE:-socialgym-dev}"

command -v ngrok >/dev/null 2>&1 || { echo "ngrok not found on PATH" >&2; exit 1; }
[[ -x "$UP_SH" ]] || { echo "not executable: $UP_SH" >&2; exit 1; }

echo "==> Bringing up the backend (scripts/up.sh $*)"
# Run from the chart dir so a bare `-f values.dev-aws.yaml` resolves the same
# way it does when up.sh is invoked directly per the chart README.
( cd "$CHART_DIR" && SOCIALGYM_NAMESPACE="$NAMESPACE" ./scripts/up.sh "$@" )

MINIKUBE_IP="$(minikube ip)"
UPSTREAM="https://${MINIKUBE_IP}:${GATEWAY_HTTPS_NODEPORT}"

# Confirm the NodePort is actually the one we're about to tunnel to.
ACTUAL_NODEPORT="$(kubectl get svc gateway -n "$NAMESPACE" \
  -o jsonpath='{.spec.ports[?(@.name=="https")].nodePort}')"
if [[ "$ACTUAL_NODEPORT" != "$GATEWAY_HTTPS_NODEPORT" ]]; then
  echo "WARNING: gateway https nodePort is $ACTUAL_NODEPORT, not $GATEWAY_HTTPS_NODEPORT." >&2
  echo "         Tunneling to $GATEWAY_HTTPS_NODEPORT anyway; set GATEWAY_HTTPS_NODEPORT to override." >&2
fi

echo
echo "==> Starting ngrok"
echo "    public : https://${NGROK_DOMAIN}"
echo "    upstream: ${UPSTREAM}  (self-signed cert, upstream TLS verification off)"
echo

# --upstream-tls-verify=false: the gateway serves the gitignored self-signed
# dev cert from scripts/gen-dev-certs.sh, so ngrok must not verify it.
exec ngrok http "$UPSTREAM" \
  --domain "$NGROK_DOMAIN" \
  --upstream-tls-verify=false
