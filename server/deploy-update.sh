#!/usr/bin/env bash
set -euo pipefail

APP_DIR="${APP_DIR:-/opt/artifact-lab}"
LOG_FILE="${LOG_FILE:-/var/log/artifact-lab-deploy.log}"

cd "$APP_DIR"

before="$(git rev-parse HEAD)"
git fetch origin main
after="$(git rev-parse origin/main)"

if [ "$before" = "$after" ]; then
  echo "$(date -Is) artifact-lab already up to date at $before" | sudo tee -a "$LOG_FILE" >/dev/null
  exit 0
fi

git reset --hard origin/main
cd "$APP_DIR/server"
sudo docker compose up -d --build --remove-orphans

echo "$(date -Is) artifact-lab updated from $before to $after" | sudo tee -a "$LOG_FILE" >/dev/null
