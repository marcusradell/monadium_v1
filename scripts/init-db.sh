#!/usr/bin/env bash
set -x
set -eo pipefail

docker run \
    --name monadium-db \
    -e POSTGRES_PASSWORD=password \
    -p 5432:5432 \
    -d \
    postgres

until pg_isready -h localhost; do
    >&2 echo "Waiting for DB."
    sleep 1
done

>&2 echo "DB started."

export DATABASE_URL=postgres://postgres:password@localhost:5432/postgres
sqlx database create
sqlx migrate run

>&2 echo "DB has been migrated."
