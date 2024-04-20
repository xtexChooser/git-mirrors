#!/usr/bin/env bash

set -e

runMW() {
	podman exec -it mediawiki "$@"
}

: "${RUN_AS_WIKI:=meta}"

(($# != 1)) && {
	echo "Usage: atre s mediawiki createwiki <WIKI ID>" >&2
	exit 1
}
echo "Creating wiki $1"

if [[ "$1" != "meta" ]]; then
	runMW php maintenance/sql.php --wiki "$RUN_AS_WIKI" --query "CREATE DATABASE wiki$1"
else
	# Bootstraping the meta wiki
	/srv/atremis/services/mariadb/monto/script/sql.sh -u mediawikiadmin --password -e 'CREATE DATABASE wikimeta;'
fi
runMW php maintenance/sql.php --wiki "$1" maintenance/tables-generated.sql
runMW php maintenance/sql.php --wiki "$1" maintenance/tables.sql
runMW php maintenance/run.php --wiki "$1" update --quick

echo "Wiki $1 initialized"
