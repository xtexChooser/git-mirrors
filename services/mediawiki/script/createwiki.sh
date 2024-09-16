#!/usr/bin/env bash

set -e

runMW() {
	podman exec -it mediawiki "$@"
}

(($# != 1)) && {
	echo "Usage: atre s mediawiki createwiki <WIKI ID>" >&2
	exit 1
}
echo "Creating wiki $1"

if [[ "$1" != "meta" ]]; then
	runMW php maintenance/sql.php --wiki "meta" --query "CREATE DATABASE wiki$1"
	runMW php maintenance/sql.php --wiki "$1" --noshared maintenance/tables-generated.sql
	runMW php maintenance/sql.php --wiki "$1" --noshared maintenance/tables.sql
	runMW php maintenance/run.php --wiki "$1" update --quick
else
	# Bootstraping the meta wiki
	/srv/atremis/services/mariadb/monto/script/sql.sh -u mediawikiadmin --password -e 'CREATE DATABASE wikimeta;'
	runMW php maintenance/sql.php --wiki "$1" maintenance/tables-generated.sql
	runMW php maintenance/sql.php --wiki "$1" maintenance/tables.sql
	runMW php maintenance/run.php --wiki "$1" update --quick
fi

echo "Wiki $1 initialized"
