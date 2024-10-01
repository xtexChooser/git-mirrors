#!/usr/bin/env bash

set -euo pipefail

echo "== MediaWiki cron jobs starting @ $(date -u)"

echo "=== Collecting wikis"
readarray -t wikis < <(jq -r 'keys[]' /srv/atremis/services/mediawiki/config/sites.json)

for wiki in "${wikis[@]}"; do
	echo "=== Processing wiki: $wiki"

	echo "==== $wiki: Running queued jobs ..."
	/srv/atremis/services/atremis/bin/atre s mediawiki maint "$wiki" runJobs

	echo "==== $wiki: Generating sitemaps ..."
	/srv/atremis/services/atremis/bin/atre s mediawiki maint "$wiki" generateSitemap \
		--memory-limit=64M \
		--fspath /var/lib/mediawiki/sitemap/"$wiki"/ \
		--urlpath=/sitemap/ \
		--skip-redirects

	echo "==== $wiki: Updating special pages ..."
	/srv/atremis/services/atremis/bin/atre s mediawiki maint "$wiki" updateSpecialPages

	echo "==== $wiki: Updating site stats ..."
	/srv/atremis/services/atremis/bin/atre s mediawiki maint "$wiki" initSiteStats \
		--memory-limit 64M \
		--update --active
done

echo "== MediaWiki cron jobs completed @ $(date -u)"
