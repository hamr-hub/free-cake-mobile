#!/bin/bash
set -euo pipefail

echo "=== Free Cake Local Dev ==="

docker compose --env-file .env.docker up --build
