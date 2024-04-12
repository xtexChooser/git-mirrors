#!/usr/bin/env bash

# Convert zone file to DNSControl script
set -e

mkdir -p out out/converted
out=$(realpath out)
temp=$(mktemp -d)

cat >"$temp"/creds.json <<EOF
{
	"bind": {
		"TYPE": "BIND"
	}
}
EOF
mkdir "$temp"/zones
ldns-read-zone \
	-e DNSKEY \
	< ../zones/"$1".zone > "$temp"/zones/"$1".zone
pushd "$temp"
dnscontrol get-zones --format=js bind - "$1" | sed -E \
	-e 's/^var DSP_BIND.*$//m' \
	-e 's/^var REG_CHANGE.*$//m' \
	-e 's/REG_CHANGEME/REG_NONE/' \
	-e 's/DnsProvider\(DSP_BIND\),//' \
	-e 's/\/\/NAMESERVER/NAMESERVER/g' \
	-e 's/DS\("@", /\/\/DS("@", /g' \
	| dnscontrol fmt -i /dev/stdin -o /dev/stdout \
	>"$out"/converted/"$1".js
popd
rm -rf "$temp"
