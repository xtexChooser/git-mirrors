#!/usr/bin/env bash

set -e

TTL=7200

echo ".	$TTL	IN	TXT		\"v=xvn_opennicdele_v1; sha256sum=$(sha256sum out/opennic-glue-strip.zone | cut -d' ' -f1)\""

echo "; OpenNIC TLDs Delegation"

readarray -t -d ' ' TLD <<<"$(grep -P '^tlds\.opennic\.glue\.\t+\d+\t+IN\t+TXT\t+' out/opennic-glue-strip.zone | cut -f5 | sed -e 's/\"//g' | tr '\n' ' ')"
for tld in "${TLD[@]}"; do
	tld="${tld//[$'\t\r\n ']/}"
	[[ "$tld" != "" ]] || continue
	[[ "$tld" != "." ]] || continue
	[[ "$tld" != "opennic.glue" ]] || continue
	echo "$tld.	$TTL	IN	NS		$tld.opennic.glue."
done
