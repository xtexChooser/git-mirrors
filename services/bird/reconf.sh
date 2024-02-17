#!/usr/bin/env bash

set -e

podman container exists bird || exit

podman exec -it bird bird -p || {
	echo "BIRD configuratio validation failed" >/dev/stderr
	exit 1
}

podman exec -it bird birdc configure
