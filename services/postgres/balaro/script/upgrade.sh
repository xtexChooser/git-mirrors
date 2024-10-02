#!/usr/bin/env bash

set -e

echo -n "Enter old version: "
read -r oldversion

echo -n "Enter new version: "
read -r newversion

mkdir -p /var/lib/postgresql/balaro{,/newdata}
sudo dinitctl stop balaro || true

time \
	podman run -it --rm --user "root:root" \
	-v /var/lib/postgresql/balaro:/var/lib/postgresql \
	-v /var/lib/postgresql/balaro/data:/var/lib/postgresql/data \
	--mount=type=image,source=codeberg.org/xens/postgres:"$oldversion",destination=/old \
	--entrypoint pg_upgrade \
	codeberg.org/xens/postgres:"$newversion" \
	-b /old/usr/local/bin/ \
	-B /usr/local/bin/ \
	-d /var/lib/postgresql/balaro \
	-D /var/lib/postgresql/balaro/newdata

echo '==== Moving data to olddata'
mv /var/lib/postgresql/balaro/{data,olddata}
echo '==== Cleaning newdata'
rm -rf /var/lib/postgresql/balaro/newdata/{postgres.conf,postgresql.conf,pg_ident.conf,pg_hba.conf}
echo '==== Moving newdata to data'
mv /var/lib/postgresql/balaro/{newdata,data}
echo '==== Updating PG_VERSION'
cp /var/lib/postgresql/balaro/data/PG_VERSION /var/lib/postgresql/balaro/PG_VERSION

echo '====== pg_upgrade finished'
echo 'After confirming, run the following code to restart PG:'
echo '   sudo dinitctl start balaro'
