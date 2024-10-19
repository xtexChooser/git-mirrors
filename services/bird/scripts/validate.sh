#!/usr/bin/env bash

set -e

make HOSTNAME=opilio.s.xvnet0.eu.org USER=root LEONIS_LOAD_ALL=y do-tpl TPL_BACKEND=bash-tpl \
	TPL_IN=services/bird/conf/bird.conf TPL_OUT=.bird-valid.conf
sed -i -e 's/include/#include/' .bird-valid.conf

podman run -it --rm --name bird-validate -v "$(pwd)":/validate \
	--privileged \
	codeberg.org/xens/bird:latest \
	-p -c /validate/.bird-valid.conf

rm -f .bird-valid.conf
