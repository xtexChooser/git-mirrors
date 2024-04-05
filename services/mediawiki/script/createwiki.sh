#!/usr/bin/env bash

set -e

runMW() {
	podman exec -it mediawiki "$@"
}

: "${RUN_AS_WIKI:=meta}"
runSQL() {
	runMW php maintenance/sql.php --wiki "$RUN_AS_WIKI" --query "$*"
}

(($# != 1)) && {
	echo "Usage: atre s mediawiki createwiki <WIKI ID>" >&2
	exit 1
}
echo "Creating wiki $1"

runSQL CREATE DATABASE wiki"$1"
runMW php maintenance/sql.php --wiki "$1" --query "ALTER DEFAULT PRIVILEGES GRANT ALL ON TABLES TO mediawiki"
runMW php maintenance/sql.php --wiki "$1" --query "ALTER DEFAULT PRIVILEGES GRANT ALL ON SEQUENCES TO mediawiki"
runMW php maintenance/sql.php --wiki "$1" maintenance/postgres/tables-generated.sql
runMW php maintenance/sql.php --wiki "$1" maintenance/postgres/tables.sql
runMW php maintenance/run.php --wiki "$1" update --quick

echo "Wiki $1 initialized"
