#!/usr/bin/env bash

set -e

runMW() {
	podman exec -it mediawiki "$@"
}

: "${RUN_AS_WIKI:=meta}"
runSQL() {
	runMW php maintenance/sql.php --wiki "$RUN_AS_WIKI" --query "$*;"
}

echo "Creating wiki $1"

runSQL CREATE DATABASE wiki"$1"
runSQL GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO mediawiki
runSQL GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO mediawiki
runMW php maintenance/sql.php --wiki "$1" maintenance/postgres/tables-generated.sql
runMW php maintenance/run.php --wiki "$1" update --quick

echo "Wiki $1 initialized"
