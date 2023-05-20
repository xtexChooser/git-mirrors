#!/usr/bin/bash

set -xe
echo "[cron.sh]"

cd ~/lydia/

echo pull deployment.git
cd deployment; git pull --force --all --ff-only; git describe --all --long; cd ..

echo run update.sh
exec ./deployment/exozyme/update.sh
