#!/usr/bin/env bash

set -x
set -eo pipefail

if ! [ -x "$(command -v psql)" ]; then
  echo "psql needs to be installed"
  exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
  echo "sqlx needs to be installed"
  echo "You can do so with: "
  echo "  cargo install --version='~0.6' sqlx-cli \
    --no-default-features --features rustls,postgres"
fi

DB_USERNAME="${DB_USERNAME:=postgres}"
DB_PASSWORD="${DB_PASSWORD:=password}"
DB_HOST="${DB_HOST:=localhost}"
DB_PORT="${DB_PORT:=5432}"
DB_NAME="${DB_NAME:=booklist}"

if [[ -z "${SKIP_DOCKER}" ]]; then
docker run -e POSTGRES_USER=${DB_USERNAME} \
           -e POSTGRES_PASSWORD=${DB_PASSWORD} \
           -e POSTGRES_DB=${DB_NAME} \
           -p "${DB_PORT}":5432 \
           -d postgres \
           postgres -N 1000
fi

export PGPASSWORD="${DB_PASSWORD}"
until psql -h "${DB_HOST}" -U "${DB_USERNAME}" -p "${DB_PORT}" -d postgres -c '\q'; do
  echo >&2 "Postgres database not ready yet. Retrying.."
  sleep 1;
done

DATABASE_URL=postgres://${DB_USERNAME}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}
export DATABASE_URL
sqlx database create
sqlx migrate run

echo "Postgres schema has been updated successfully"
