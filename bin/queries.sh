#!/usr/bin/env bash

set -o errexit

export DATABASE_URL=$(bin/postgres.sh database-url)
make postgres

sqlx database setup
cargo sqlx prepare -- --all-targets --all-features --tests