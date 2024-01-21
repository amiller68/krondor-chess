#!/usr/bin/env bash

set -o errexit

export DATABASE_URL="postgres://postgres:postgres@localhost:5432/postgres"

make postgres

cargo shuttle run