#!/usr/bin/env bash
set -e

ELLA_DEV_HOME=${ELLA_DEV_HOME:=$(pwd)}
DOCKER_COMPOSE_FLAGS=${DOCKER_COMPOSE_FLAGS:=""}

if [[ -z "${ELLA_DEV_HOME}" ]]; then
  echo "ELLA_DEV_HOME environment variable is not set!" && exit 255
fi

# shellcheck disable=SC2068
cd "$ELLA_DEV_HOME" &&
  docker-compose \
    -f docker-compose.yml ${DOCKER_COMPOSE_FLAGS} \
    --project-name ella \
    --project-directory . $@