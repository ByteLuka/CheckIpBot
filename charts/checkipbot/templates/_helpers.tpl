{{- define "checkipbot.name" -}}
{{- .Chart.Name }}
{{- end }}

{{- define "checkipbot.fullname" -}}
{{- printf "%s-%s" .Release.Name .Chart.Name | trunc 63 | trimSuffix "-" }}
{{- end }}

{{- define "checkipbot.labels" -}}
helm.sh/chart: {{ .Chart.Name }}-{{ .Chart.Version }}
app.kubernetes.io/name: {{ include "checkipbot.name" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
app.kubernetes.io/version: {{ .Values.image.tag | default .Chart.AppVersion | quote }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
{{- end }}

{{- define "checkipbot.selectorLabels" -}}
app.kubernetes.io/name: {{ include "checkipbot.name" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end }}

{{- define "checkipbot.tokenSecretName" -}}
{{- if .Values.discord.existingSecret }}
{{- .Values.discord.existingSecret }}
{{- else }}
{{- .Values.tokenSecret.name }}
{{- end }}
{{- end }}
