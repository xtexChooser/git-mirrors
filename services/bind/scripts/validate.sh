#!/usr/bin/env bash

set -e

make HOSTNAME=opilio.s.xvnet0.eu.org USER=root LEONIS_LOAD_ALL=y do-tpl TPL_BACKEND=bash-tpl \
	TPL_IN=services/bind/conf/named.conf TPL_OUT=.bind-valid.conf
podman image exists codeberg.org/xens/dns-root-zone:latest || podman image pull codeberg.org/xens/dns-root-zone:latest
(
	while true; do
		if [[ "$(podman container inspect bind-validate 2>/dev/null | jq '.[0].State.Status' -r)" == 'running' ]]; then
			podman container rm --force bind-validate >/dev/null
			exit
		fi
	done
) &
podman run -it --rm --name bind-validate -v "$(pwd)":/validate \
	--mount=type=image,source=codeberg.org/xens/dns-root-zone:latest,destination=/opt/root-zone \
	codeberg.org/xens/bind:latest \
	named -c /validate/.bind-valid.conf -g | tee .bind-valid.log
succ=true
if grep -F 'exiting (due to fatal error)' .bind-valid.log; then
	succ=''
fi
rm -f .bind-valid.log
if [[ -z "$succ" ]]; then
	echo 'Validation failed' >/dev/stderr
	exit 1
else
	rm -f .bind-valid.conf
fi
exit 0
