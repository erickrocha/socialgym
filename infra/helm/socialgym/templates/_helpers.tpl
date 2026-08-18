{{/*
Resolve a secret value that must stay stable across `helm upgrade` without
ever being committed to git: reuse the value already stored in the named
Secret's key if it exists, otherwise fall back to a values-provided
override, otherwise generate a fresh random string.
Usage: {{ include "socialgym.secretValue" (list . "postgres" "POSTGRES_PASSWORD" .Values.postgres.password 24) }}
*/}}
{{- define "socialgym.secretValue" -}}
{{- $ctx := index . 0 -}}
{{- $secretName := index . 1 -}}
{{- $key := index . 2 -}}
{{- $override := index . 3 -}}
{{- $length := index . 4 -}}
{{- $existing := lookup "v1" "Secret" $ctx.Release.Namespace $secretName -}}
{{- if $existing -}}
{{- index $existing.data $key | b64dec -}}
{{- else if $override -}}
{{- $override -}}
{{- else -}}
{{- randAlphaNum (int $length) -}}
{{- end -}}
{{- end -}}

{{/*
Common labels applied to every resource in this chart.
*/}}
{{- define "socialgym.labels" -}}
app.kubernetes.io/part-of: socialgym
app.kubernetes.io/managed-by: {{ .Release.Service }}
{{- end -}}
