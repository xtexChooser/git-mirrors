#!/usr/bin/env bash
set -e
podman exec -it balaro psql -U postgres
