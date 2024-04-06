#!/usr/bin/env bash

set -e

mkdir -p /var/lib/mariadb/monto
podman run -it --rm --user "root:root" \
	-v /var/lib/mariadb/monto:/var/lib/mariadb \
	--entrypoint "/usr/local/script/mariadb-install-db" \
	codeberg.org/xvnet/mariadb \
	--skip-test-db \
	--user=root
