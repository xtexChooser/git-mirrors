#!/usr/bin/env bash
set -e
exec podman exec -it monto /usr/local/mysql/bin/mariadb -u root "$@"
