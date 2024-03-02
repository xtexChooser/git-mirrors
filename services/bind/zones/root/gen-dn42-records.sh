#!/usr/bin/env bash

set -e

TTL=7200
DN42_ZONES=("dn42."
	"20.172.in-addr.arpa." "21.172.in-addr.arpa." "22.172.in-addr.arpa." "23.172.in-addr.arpa."
	"10.in-addr.arpa." "d.f.ip6.arpa.")

echo ".	$TTL	IN	TXT		\"v=xvn_dn42dele_v1; sha256sum=$(sha256sum out/dn42-delegation.json | cut -d' ' -f1)\""

echo "; DN42 Delegation Servers"

jq -r '.[].Attributes.[] | select(.[0] == "nserver") | .[1]' out/dn42-delegation.json |
	sort | uniq |
	while read -r line; do
		echo "; nserver: $line"
		zone="$(cut -d' ' -f1 <<<"$line")"
		rdata="$(cut -d' ' -f2 <<<"$line")"
		if grep -qF ':' <<<"$rdata"; then
			echo "$zone.	$TTL	IN	AAAA	$rdata"
		else
			echo "$zone.	$TTL	IN	A		$rdata"
		fi
	done

jq -r '.[].Attributes.[] | select(.[0] == "ds-rdata") | .[1]' out/dn42-delegation.json |
	sort | uniq |
	while read -r line; do
		echo "; ds-rdata: $line"
		echo "delegation-servers.dn42.	$TTL	IN	DS		$line"
	done

echo "; DN42 Zones"

jq -r '.[].Attributes.[] | select(.[0] == "nserver") | .[1] | split(" ")[0]' out/dn42-delegation.json |
	sort | uniq |
	while read -r addr; do
		echo "; Delegation server: $addr"
		for zone in "${DN42_ZONES[@]}"; do
			echo "$zone		$TTL	IN	NS		$addr"
		done
	done
