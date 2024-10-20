# shellcheck shell=bash

atre::postgres::balaro::sql() {
	podman exec -it balaro psql -U postgres "$@"
	return
}

atre::postgres::balaro::initdb() {
	atre::publog "Start initializing Balaro database ..."

	local password
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

	atre::publog "End initializing Balaro database ..."
	return
}

atre::postgres::balaro::upgrade() {
	atre::publog "Start upgrading Balaro database ..."

	local oldversion
	echo -n "Enter old version: "
	read -r oldversion

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

	atre::publog "End upgrading Balaro database ..."

	echo '====== pg_upgrade finished'
	echo 'After confirming, run the following code to restart PG:'
	echo '   sudo dinitctl start balaro'
	echo '   sudo rm -rf /var/lib/postgresql/balaroold'
}
