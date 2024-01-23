#!/usr/bin/env bash

set -o errexit

export DATABASE_URL=$(bin/postgres.sh database-url)
make postgres

cargo shuttle run