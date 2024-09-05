#!/usr/bin/env bash

set -e

make HOSTNAME=opilio.s.xvnet0.eu.org USER=root LEONIS_LOAD_ALL=y do-tpl TPL_BACKEND=bash-tpl \
	TPL_IN=services/caddy/Caddyfile TPL_OUT=.caddy-valid.Caddyfile
sed -i -e 's/include/#include/' .caddy-valid.Caddyfile

podman run -it --rm --name caddy-validate -v "$(pwd)":/validate \
	codeberg.org/xens/x-caddy:latest \
	caddy validate --config /validate/.caddy-valid.Caddyfile --adapter caddyfile

rm -f .caddy-valid.Caddyfile
