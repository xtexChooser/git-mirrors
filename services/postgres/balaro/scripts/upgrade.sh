#!/usr/bin/env bash

set -eo pipefail

echo -n "Enter old version: "
read -r oldversion

set -ux

dinitctl stop balaro
mkdir -vp /var/lib/postgresql/balaroold
mv -v /var/lib/postgresql/balaro{,old}/data
cp -v /var/lib/postgresql/balaro/* /var/lib/postgresql/balaroold/

podman image pull codeberg.org/xens/postgres:"$oldversion"
podman run -it -d --user "root:root" \
	--name balaroold \
	-v /var/lib/postgresql/balaroold:/var/lib/postgresql \
	-v /var/lib/postgresql/balaroold/data:/var/lib/postgresql/data \
	--publish=5632:5432/tcp \
	codeberg.org/xens/postgres:"$oldversion"

time atre svc postgres/balaro initdb
dinitctl start balaro

time (
	podman exec -it balaroold pg_dumpall | podman exec -it balaro psql -d postgres -p 5433
)
dinitctl stop balaro
podman stop balaroold

echo '====== pg_upgrade finished'
echo 'After confirming, run the following code to restart PG:'
echo '   sudo dinitctl start balaro'
echo '   sudo rm -rf /var/lib/postgresql/balaroold'
