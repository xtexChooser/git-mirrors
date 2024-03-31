#!/usr/bin/env bash

set -e

echo -n "Enter password: "
read -r password

mkdir -p /var/lib/postgresql/balaro{,/data}
podman run -it --rm --user "root:root" \
	-v /var/lib/postgresql/balaro:/var/lib/postgresql \
	-v /var/lib/postgresql/balaro/data:/var/lib/postgresql/data \
	-e POSTGRES_PASSWORD="$password" \
	codeberg.org/xvnet/postgres
