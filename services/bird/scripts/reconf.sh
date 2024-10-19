#!/usr/bin/env bash
set -e
podman container exists bird || exit

podman exec -it bird bird -p || {
	echo "BIRD configuration validation failed" >/dev/stderr
	exit 1
}

echo 'Reconfiguring BIRD...'
podman exec -it bird birdc configure
echo 'Reconfigured BIRD'
