#! /usr/bin/env bash
set -x
set -eo pipefail

# check for psql, sqlx binaries
if ! [ -x "$(command -v psql)" ]; then
  echo >&2 "Error: \'psql\' is not installed."
  exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 "Error: \'sqlx\' is not installed."
  echo >&2 "Use: "
  echo >&2 "    cargo install --version=0.6.0 sqlx-cli --features postgres"
  exit 1
fi

# db credentials
DB_USER=${POSTGRES_USER:=ruckus}
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="${POSTGRES_DB:=ruckus_db}"
DB_PORT="${POSTGRES_PORT:=5432}"

# reattempt connection until ok
export PGPASSWORD="${DB_PASSWORD}"
until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "${DB_NAME}" -c '\q' ; do
  >&2 echo "Postgres connection unavailable, recheking"
  sleep 2
done

>&1 echo "Postgres is up and running on port ${DB_PORT}"

# migrate
export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
sqlx database create
sqlx migrate run

>&1 echo "Migrations complete"
