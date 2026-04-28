#!/bin/bash
set -euo pipefail

DEPLOY_DIR="/opt/free-cake"
ENV_FILE=".env.docker"

echo "=== Free Cake Deploy ==="

cd "$DEPLOY_DIR"

git pull origin main

docker compose build --parallel

docker compose down --timeout 30

docker compose up -d --remove-orphans

echo "=== Waiting for services ==="
sleep 10

docker compose ps

echo "=== Health checks ==="
for svc in server client mysql redis; do
  STATUS=$(docker compose ps --format json "$svc" 2>/dev/null | jq -r '.Health // "unknown"' || echo "unknown")
  echo "$svc: $STATUS"
done

echo "=== Deploy complete ==="
