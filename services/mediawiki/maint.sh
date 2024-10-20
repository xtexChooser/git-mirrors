# shellcheck shell=bash

atre::mediawiki::maint() {
	if [[ $# -lt 2 ]]; then
		echo "usage: atre s mediawiki maint <WIKI> <SCRIPT> [ARGUMENTS]" >&2
		return 1
	fi
	local wiki="$1" script="$2"
	shift 2
	podman exec -it mediawiki php maintenance/run.php "$script" --wiki "$wiki" "$@"
	return
}

atre::mediawiki::shell() {
	atre::mediawiki::maint "$1" shell
	return
}

atre::mediawiki::update() {
	atre::publog "MW: Running update: $1" \
		"MediaWiki-Wiki: $1"
	atre::mediawiki::maint "$1" update --quick
	return
}

atre::mediawiki::delwiki() {
	(($# != 1)) && {
		echo "usage: atre s mediawiki delwiki <WIKI ID>" >&2
		return 1
	}
	echo "Deleting wiki: $1"
	atre::publog "MW: Deleting wiki: $1" \
		"MediaWiki-Wiki: $1"
	atre::mediawiki::maint meta sql --query "DROP DATABASE wiki$1;"
	atre::publog "MW: Deleted wiki: $1" \
		"MediaWiki-Wiki: $1"
	echo "Deleted wiki: $1"
	return
}

atre::mediawiki::addwiki() {
	(($# != 1)) && {
		echo "usage: atre s mediawiki addwiki <WIKI ID>" >&2
		return 1
	}
	local wiki="$1" domain
	domain="$(jq -r ".$wiki" /srv/atremis/services/mediawiki/config/sites.json)"
	echo "Found domain: $domain"
	[[ -n "$wiki" ]] || atre::error "Invalid wiki ID"
	[[ -n "$domain" ]] || atre::error "Invalid wiki domain"
	[[ -e /srv/atremis/services/mediawiki/config/sites/SiteSettings."$wiki".php ]] || atre::error "Wiki site settings not found"

	echo "Creating wiki $wiki, at $domain"
	atre::publog "MW: Creating wiki: $wiki" \
		"MediaWiki-Wiki: $wiki"

	if [[ "$wiki" == "meta" ]]; then
		# Bootstraping the meta wiki
		atre s mariadb/monto sql -u mediawikiadmin --password -e 'CREATE DATABASE wikimeta;'
		atre::mediawiki::maint "$wiki" sql maintenance/tables-generated.sql
		atre::mediawiki::maint "$wiki" sql maintenance/tables.sql
		atre::mediawiki::update "$wiki"
	else
		# Creating normal wikis
		atre::mediawiki::maint meta sql --query "CREATE DATABASE wiki$wiki"
		atre::mediawiki::maint "$wiki" sql maintenance/tables-generated.sql
		atre::mediawiki::maint "$wiki" sql maintenance/tables.sql
		atre::mediawiki::update "$wiki"
	fi

	atre::mediawiki::maint meta addSite wiki"$wiki" xvwiki \
		--pagepath "https://$domain/w/\$1" \
		--filepath "https://$domain/\$1"

	atre::publog "MW: Created wiki: $wiki" \
		"MediaWiki-Wiki: $wiki"
	echo "Wiki $1 is created"
}

atre::mediawiki::addcargo() {
	(($# != 1)) && {
		echo "usage: atre s mediawiki addcargo <WIKI ID>" >&2
		return 1
	}
	local wiki="$1"
	[[ -n "$wiki" ]] || atre::error "Invalid wiki ID"
	[[ -e /srv/atremis/services/mediawiki/config/sites/SiteSettings."$wiki".php ]] || atre::error "Wiki site settings not found"

	echo "Creating cargo database $wiki, at $domain"
	atre::publog "MW: Creating cargo database for $wiki" \
		"MediaWiki-Wiki: $wiki"

	atre::mediawiki::maint meta sql --query "CREATE DATABASE wikicargo$wiki"

	atre::publog "MW: Created cargo database for $wiki" \
		"MediaWiki-Wiki: $wiki"
	echo "Wiki cargo database created for $1"
}
