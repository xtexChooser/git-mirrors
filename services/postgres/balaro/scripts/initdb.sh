#!/usr/bin/env bash

set -e

echo -n "Enter password: "
read -r password

mkdir -p /var/lib/postgresql/balaro{,/data}
podman run -it --rm --user "root:root" \
	-v /var/lib/postgresql/balaro/data:/var/lib/postgresql/data \
	-e PGDATA=/var/lib/postgresql/data \
	-e POSTGRES_PASSWORD="$password" \
	codeberg.org/xens/postgres || true
rm -rf /var/lib/postgresql/balaro/data/{postgres.conf,postgresql.conf,pg_ident.conf,pg_hba.conf}
cp /var/lib/postgresql/balaro/data/PG_VERSION /var/lib/postgresql/balaro/PG_VERSION
