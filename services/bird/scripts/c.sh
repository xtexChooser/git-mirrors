#!/usr/bin/env bash
set -e
exec podman exec -it bird birdc "$@"
