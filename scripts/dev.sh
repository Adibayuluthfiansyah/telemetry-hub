#!/usr/bin/env bash
set -Eeuo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
COMPOSE_FILE="$ROOT_DIR/docker/docker-compose.yml"
SERVER_PID=""
SIMULATOR_PID=""
FRONTEND_PID=""
CLEANED_UP=false
cleanup() {
  if $CLEANED_UP; then
    return
  fi
  CLEANED_UP=true
  echo
  echo "Shutting down development stack..."
  if [[ -n "${FRONTEND_PID}" ]] && kill -0 "${FRONTEND_PID}" 2>/dev/null; then
    kill "${FRONTEND_PID}" 2>/dev/null || true
  fi
  if [[ -n "${SIMULATOR_PID}" ]] && kill -0 "${SIMULATOR_PID}" 2>/dev/null; then
    kill "${SIMULATOR_PID}" 2>/dev/null || true
  fi
  if [[ -n "${SERVER_PID}" ]] && kill -0 "${SERVER_PID}" 2>/dev/null; then
    kill "${SERVER_PID}" 2>/dev/null || true
  fi
  wait "${FRONTEND_PID}" 2>/dev/null || true
  wait "${SIMULATOR_PID}" 2>/dev/null || true
  wait "${SERVER_PID}" 2>/dev/null || true
  echo "Development stack stopped."
}
trap cleanup INT TERM EXIT
cd "$ROOT_DIR"
echo "Starting PostgreSQL..."
docker compose -f "$COMPOSE_FILE" up -d postgres
echo "Waiting for PostgreSQL..."

until docker exec telemetry-postgres pg_isready \
  -U "${POSTGRES_USER:-postgres}" \
  -d "${POSTGRES_DB:-telemetry}" \
  >/dev/null 2>&1; do
  sleep 1
done
echo "PostgreSQL is ready."
echo "Starting server..."
cargo run -p server &
SERVER_PID=$!
echo "Waiting for server..."

until curl -fsS http://localhost:3000/api/v1/health >/dev/null 2>&1; do
  if ! kill -0 "$SERVER_PID" 2>/dev/null; then
    echo "Server exited unexpectedly."
    exit 1
  fi

  sleep 1
done

echo "Server is ready."
echo "Starting frontend..."
cd "$ROOT_DIR/frontend"
npm run dev &
FRONTEND_PID=$!
echo "Waiting for frontend..."

until curl -fsS http://localhost:3001 >/dev/null 2>&1; do
  if ! kill -0 "$FRONTEND_PID" 2>/dev/null; then
    echo "Frontend exited unexpectedly."
    exit 1
  fi
  sleep 1
done

echo "Frontend is ready."
echo "Starting simulator..."
cd "$ROOT_DIR"
cargo run -p simulator &
SIMULATOR_PID=$!
echo "Development stack is running."
echo "Press Ctrl-C to stop server, simulator, and frontend."
wait