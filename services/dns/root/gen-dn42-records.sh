#!/usr/bin/env bash

set -e

TTL=7200

echo ".	$TTL	IN	TXT		\"v=xvn_dn42dele_v1; sha256sum=$(sha256sum out/dn42-delegation-orig.zone | cut -d' ' -f1)\"; \
commit=$(grep -oE '^;; Commit Reference: (.*)$' out/dn42-delegation-orig.zone | cut -d ' ' -f4)"

while read -r line; do
	echo "${line/[$'\t ']IN[$'\t ']/$'\t'$TTL$'\t'IN$'\t'}"
done < out/dn42-delegation-orig.zone
