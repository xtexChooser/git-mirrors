#!/usr/bin/bash
#
# Updater entrypoint
# should be called by toolforge-jobs
#

set -xe
echo "[cron.sh]"

echo pull deployment.git
cd deployment; git pull --force --all --ff-only; git describe --all --long; cd ..

echo run update.sh
exec ./deployment/toolforge/update.sh
