{{/* vim: set filetype=mustache: */}}
{{/*
Expand the name of the chart.
*/}}
{{- define "auth-api.name" -}}
{{- default .Chart.Name .Values.nameOverride | trunc 63 | trimSuffix "-" -}}
{{- end -}}

{{/*
Create a default fully qualified app name.
We truncate at 63 chars because some Kubernetes name fields are limited to this (by the DNS naming spec).
If release name contains chart name it will be used as a full name.
*/}}
{{- define "auth-api.fullname" -}}
{{- if .Values.fullnameOverride -}}
{{- .Values.fullnameOverride | trunc 63 | trimSuffix "-" -}}
{{- else -}}
{{- $name := default .Chart.Name .Values.nameOverride -}}
{{- if contains $name .Release.Name -}}
{{- .Release.Name | trunc 63 | trimSuffix "-" -}}
{{- else -}}
{{- printf "%s-%s" .Release.Name $name | trunc 63 | trimSuffix "-" -}}
{{- end -}}
{{- end -}}
{{- end -}}

{{/*
Checks if the deployment is using sqlite
*/}}
{{- define "auth-api.is-sqlite" -}}
{{- if and (and .Values.env .Values.env.DATABASE_URL) (hasPrefix "sqlite:" .Values.env.DATABASE_URL) -}}
{{- "true" -}}
{{- else if and (and .Values.config (and .Values.config.database .Values.config.database.url)) (hasPrefix "sqlite:" .Values.config.database.url) -}}
{{- "true" -}}
{{- else -}}
{{- "false" -}}
{{- end -}}
{{- end -}}

{{/*
Gets sqlite path
*/}}
{{- define "auth-api.sqlite-path" -}}
{{- $env := default dict .Values.env -}}
{{- $config := default dict .Values.config -}}
{{- $database := default dict $config.database -}}
{{- trimPrefix "sqlite:" (default $env.DATABASE_URL (default "" $database.url)) -}}
{{- end -}}

{{/*
Create chart name and version as used by the chart label.
*/}}
{{- define "auth-api.chart" -}}
{{- printf "%s-%s" .Chart.Name .Chart.Version | replace "+" "_" | trunc 63 | trimSuffix "-" -}}
{{- end -}}

{{/*
Common labels
*/}}
{{- define "auth-api.labels" -}}
helm.sh/chart: {{ include "auth-api.chart" . }}
{{ include "auth-api.selectorLabels" . }}
{{- if .Chart.AppVersion }}
app.kubernetes.io/version: {{ .Chart.AppVersion | quote }}
{{- end }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
{{- end -}}

{{/*
Selector labels
*/}}
{{- define "auth-api.selectorLabels" -}}
app.kubernetes.io/name: {{ include "auth-api.name" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end -}}
