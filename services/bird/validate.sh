#!/usr/bin/env bash

set -e

make HOSTNAME=nl-alk1.svr.xvnet.eu.org USER=root LEONIS_LOAD_ALL=y do-tpl TPL_BACKEND=bash-tpl \
	TPL_IN=services/bird/conf/bird.conf TPL_OUT=.bird-valid.conf
sed -i -e 's/include/#include/' .bird-valid.conf
bird -p -c .bird-valid.conf
rm -f .bird-valid.conf
