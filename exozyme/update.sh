#!/usr/bin/bash
#
# Updater script
#

set -xe
echo "[update.sh]"
pwd

echo pull lydia.git in ./src

cd src
git_head_commit=$(git rev-parse HEAD)
echo current git HEAD: "$git_head_commit"

git pull --force --all --ff-only; git describe --all --long

if [[ $git_head_commit == $(git rev-parse HEAD) ]]; then
    echo no changes got
    if [[ -z "$LYDIA_FORCE_UPDATE" ]]; then
        exit
    fi
else
    echo new HEAD: "$(git rev-parse HEAD)"
fi
cd ..

echo update systemd units
cp d/lydia.service ~/.config/systemd/user/lydia.service
cp d/lydia-update.timer ~/.config/systemd/user/lydia-update.timer
systemctl --user daemon-reload

exec systemctl --user restart lydia.service
