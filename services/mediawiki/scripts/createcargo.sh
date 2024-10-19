#!/usr/bin/env bash

set -e

runMW() {
	podman exec -it mediawiki "$@"
}

(($# != 1)) && {
	echo "Usage: atre s mediawiki createcargo <WIKI ID>" >&2
	exit 1
}
echo "Creating cargo database for wiki $1"

runMW php maintenance/sql.php --wiki "$1" --query "CREATE DATABASE wikicargo$1"

echo "Cargo database for wiki $1 initialized"
