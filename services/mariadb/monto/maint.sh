# shellcheck shell=bash

atre::mariadb::monto::sql() {
	podman exec -it monto /usr/local/mysql/bin/mariadb -u root "$@"
	return
}

atre::mariadb::monto::initdb() {
	atre::publog "Start initializing Monto database ..."
	mkdir -p /var/lib/mariadb/monto
	podman run -it --rm --user "root:root" \
		-v /var/lib/mariadb/monto:/var/lib/mariadb \
		--entrypoint "/usr/local/mysql/scripts/mariadb-install-db" \
		codeberg.org/xens/mariadb \
		--skip-test-db \
		--user=root
	atre::publog "End initializing Monto database ..."
	return
}
