## Installation

To install this application using Helm run the following commands: 

```bash
helm repo add jorritsalverda https://helm.jorritsalverda.com
kubectl create namespace jarvis-wallbox-exporter

helm upgrade \
  jarvis-wallbox-exporter \
  jorritsalverda/jarvis-wallbox-exporter \
  --install \
  --namespace jarvis-wallbox-exporter \
  --set secret.gcpServiceAccountKeyfile='{abc: blabla}' \
  --wait
```
