#!/usr/bin/env bash

set -e

curl \
	-H "Authorization: Bearer $(cat ../ntfytoken.txt)" \
	-H "X-Title: External DNS pushed" \
	-H "X-Tags: pipline-push-dns,ext-dns-pushed" \
	-d "$(cat dnspush.log)" \
	-SL --retry 3 \
	https://ntfy.xvnet.eu.org/publogs
