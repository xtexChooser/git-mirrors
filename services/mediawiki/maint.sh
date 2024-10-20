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
	atre::publog "MW: $1: Running update script" \
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
	atre::publog "MW: $1: Deleting wiki" \
		"MediaWiki-Wiki: $1"
	atre::mediawiki::maint meta sql --query "DROP DATABASE wiki$1;"
	atre::publog "MW: $1: Deleted wiki" \
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
	atre::publog "MW: $wiki: Creating wiki" \
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

	atre::publog "MW: $wiki: Created wiki" \
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
	atre::publog "MW: $wiki: Creating cargo database" \
		"MediaWiki-Wiki: $wiki"

	atre::mediawiki::maint meta sql --query "CREATE DATABASE wikicargo$wiki"

	atre::publog "MW: $wiki: Created cargo database" \
		"MediaWiki-Wiki: $wiki"
	echo "Wiki cargo database created for $1"
}

atre::mediawiki::allwikis() {
	jq -r 'keys[]' /srv/atremis/services/mediawiki/config/sites.json
	return
}

atre::mediawiki::cronjob() {
	local wiki="${1:-}"
	if [[ -z "$wiki" ]]; then
		echo "== MediaWiki cron jobs starting @ $(date -u)"

		echo "=== Collecting wikis"
		readarray -t wikis < <(atre::mediawiki::allwikis)
		for wiki in "${wikis[@]}"; do
			echo "=== Processing wiki: $wiki"
			atre::mediawiki::cronjob "$wiki"
		done

		echo "== MediaWiki cron jobs completed @ $(date -u)"
	else
		atre::publog "MW: $wiki: Started cron jobs" \
			"MediaWiki-Wiki: $wiki"

		echo "==== $wiki: Running queued jobs ..."
		atre::mediawiki::maint "$wiki" runJobs

		echo "==== $wiki: Generating sitemaps ..."
		atre::mediawiki::maint "$wiki" generateSitemap \
			--memory-limit=128M \
			--fspath /var/lib/mediawiki/sitemap/"$wiki"/ \
			--urlpath=/sitemap/ \
			--skip-redirects

		echo "==== $wiki: Updating special pages ..."
		atre::mediawiki::maint "$wiki" updateSpecialPages

		echo "==== $wiki: Updating site stats ..."
		atre::mediawiki::maint "$wiki" initSiteStats \
			--memory-limit 128M \
			--update \
			--active

		atre::publog "MW: $wiki: Finished cron jobs" \
			"MediaWiki-Wiki: $wiki"
	fi
}
