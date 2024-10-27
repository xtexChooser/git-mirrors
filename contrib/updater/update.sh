#!/usr/bin/env bash

set -e
apk add curl jq git yq coreutils git-subtree jo \
	php{,-calendar,-ctype,-dom,-fileinfo,-iconv,-intl,-xml,-mbstring,-xmlreader} \
	composer npm

git config --global user.name "xvbot-mwupdater"
git config --global user.email "xvbot+codeberg@xvnet.eu.org"

function doUpdate() {
	scripts/update
	scripts/updatetweaks
	git remote set-url origin "https://xvbot:$CODEBERG_TOKEN@codeberg.org/xens/mediawiki.git"
	git push --force origin HEAD:bot/update

	# create a PR
	pulls="$(curl -X 'GET' \
		'https://codeberg.org/api/v1/repos/xens/mediawiki/pulls?state=open&labels=239913' \
		-H 'Accept: application/json' \
		-SL --retry 2 |
		jq '. | length')"
	if [[ "$pulls" == "0" ]]; then
		curl -X 'POST' \
			'https://codeberg.org/api/v1/repos/xens/mediawiki/pulls' \
			-H 'Accept: application/json' \
			-H 'Content-Type: application/json' \
			-H "Authorization: token $CODEBERG_TOKEN" \
			-SL --retry 2 \
			-d "$(jo -- base=main head="bot/update" title="[bot] Merge upstream" \
				body="$(printf 'Updated-at: %s\nCI-Link: <%s>' "$(date -u)" "$CI_STEP_URL")" \
				labels="$(jo -a -- -n 239913)")"
	fi
}

if doUpdate; then
	curl \
		-H "Authorization: Bearer $NTFY_TOKEN" \
		-H "X-Title: MediaWiki auto-update succeeded" \
		-H "X-Actions: view, View on CI, $CI_STEP_URL" \
		-H "X-Tags: mediawiki,mwupdater,pipeline-success" \
		-H "X-Priority: min" \
		-d "$(printf 'MW-Updater-Status: success\nMW-Updater-HEAD: %s\nCI-Link: <%s>' "$(git rev-parse HEAD)" "$CI_STEP_URL")" \
		-SL --retry 2 \
		https://ntfy.xvnet.eu.org/publogs

	# close warning issues
	curl -X 'GET' \
		'https://codeberg.org/api/v1/repos/xens/mediawiki/issues?state=open&labels=bot%2Fupdate-fail&type=issues&limit=1' \
		-H 'Accept: application/json' |
		jq -r '.[] | .url' |
		while read -r issue; do
			curl -X 'PATCH' \
				"$issue" \
				-H 'Accept: application/json' \
				-H 'Content-Type: application/json' \
				-H "Authorization: token $CODEBERG_TOKEN" \
				-SL --retry 2 \
				-d "$(jo -- state=close)"
		done
else
	curl \
		-H "Authorization: Bearer $NTFY_TOKEN" \
		-H "X-Title: MediaWiki auto-update failed" \
		-H "X-Actions: view, View on CI, $CI_STEP_URL" \
		-H "X-Tags: mediawiki,mwupdater,pipeline-failure" \
		-H "X-Priority: min" \
		-d "$(printf 'MW-Updater-Status: failure\nCI-Link: <%s>' "$CI_STEP_URL")" \
		-SL --retry 2 \
		https://ntfy.xvnet.eu.org/publogs

	# create a warning issue
	issues="$(curl -X 'GET' \
		'https://codeberg.org/api/v1/repos/xens/mediawiki/issues?state=open&labels=bot%2Fupdate-fail&type=issues&limit=1' \
		-H 'Accept: application/json' \
		-SL --retry 2 |
		jq '. | length')"
	if [[ "$issues" == "0" ]]; then
		curl -X 'POST' \
			'https://codeberg.org/api/v1/repos/xens/mediawiki/issues' \
			-H 'Accept: application/json' \
			-H 'Content-Type: application/json' \
			-H "Authorization: token $CODEBERG_TOKEN" \
			-SL --retry 2 \
			-d "$(jo title="[bot] MW Auto-updater fails" \
				body="$(printf 'CI-Link: <%s>' "$CI_STEP_URL")" \
				labels="$(jo -a -- -n 239885)" assignees="$(jo -a -- -s xtex)")"
	fi
fi
