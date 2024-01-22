#!/usr/bin/env bash

set -o errexit
set -o nounset

POSTGRES_CONTAINER_NAME="krondor-chess-postgres"
POSTGRES_VOLUME_NAME="krondor-chess-postgres-data"

POSTGRES_ROOT_USER="postgres"
POSTGRES_ROOT_PASSWORD="postgres"

CONTAINER_RUNTIME="podman"
if which docker &>/dev/null; then
	CONTAINER_RUNTIME="docker"
fi

function run {
	start-postgres-container
}

# Helpers:

function start-postgres-container {
	ensure-postgres-container-exists
	${CONTAINER_RUNTIME} start ${POSTGRES_CONTAINER_NAME}
}

function ensure-postgres-container-exists {
	docker pull postgres
	create-postgres-container
}

function create-postgres-container {
	if ${CONTAINER_RUNTIME} ps -a | grep ${POSTGRES_CONTAINER_NAME} &>/dev/null; then
		return
	fi

	${CONTAINER_RUNTIME} volume create ${POSTGRES_VOLUME_NAME} || true
	
	${CONTAINER_RUNTIME} run \
		--name ${POSTGRES_CONTAINER_NAME} \
		--env POSTGRES_USER=${POSTGRES_ROOT_USER} \
		--env POSTGRES_PASSWORD=${POSTGRES_ROOT_PASSWORD} \
		--env POSTGRES_DB=postgres \
		--publish 5432:5432 \
		--volume ${POSTGRES_VOLUME_NAME}:/var/lib/postgresql/data \
		--detach \
		postgres
}


function clean() {
	docker stop ${POSTGRES_CONTAINER_NAME} || true
	${CONTAINER_RUNTIME} rm -fv ${POSTGRES_CONTAINER_NAME} || true
	${CONTAINER_RUNTIME} volume rm -f ${POSTGRES_VOLUME_NAME} || true
}

$1
