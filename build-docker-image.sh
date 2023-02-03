#!/usr/bin/env sh

export DOCKER_BUILDKIT=1

docker build --pull -t axum-ok .
