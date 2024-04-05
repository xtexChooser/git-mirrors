#!/usr/bin/env bash
set -e
exec podman exec -it balaro psql -U postgres "$@"
