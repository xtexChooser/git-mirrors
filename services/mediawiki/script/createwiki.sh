#!/usr/bin/env bash

set -euxo pipefail

runMW() {
	podman exec -it mediawiki "$@"
}

(($# != 2)) && {
	echo "Usage: atre s mediawiki createwiki <WIKI ID> <DOMAIN>" >&2
	exit 1
}
domain="$(jq -r ".$1" /srv/atremis/services/mediawiki/config/sites.json)"
echo "Creating wiki $1 at $domain"
echo "---------- To revert changes: sudo podman exec -it mediawiki php maintenance/sql.php --wiki \"meta\" --query \"DROP DATABASE wiki$1;\""
[[ -n "$domain" ]] || exit 1
[[ -e /srv/atremis/services/mediawiki/config/sites/SiteSettings."$1".php ]] || exit 1

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

runMW php maintenance/run.php --wiki "meta" addSite wiki"$1" xvwiki \
	--pagepath "https://$domain/w/\$1" \
	--filepath "https://$domain/\$1"

echo "Wiki $1 initialized"
