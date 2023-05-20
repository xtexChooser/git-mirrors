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

echo generate lydia.service
tee ~/.config/systemd/user/lydia.service << EOF
[Unit]
Description=lydia server

[Service]
ExecStart=$(pwd)/deployment/exozyme/run.sh

[Install]
WantedBy=default.target
EOF

echo generate lydia-update.service
tee ~/.config/systemd/user/lydia-update.service << EOF
[Unit]
Description=lydia updater

[Service]
ExecStart=$(pwd)/deployment/exozyme/cron.sh
EOF

echo generate lydia-update.timer
tee ~/.config/systemd/user/lydia-update.timer < d/lydia-update.timer
systemctl --user daemon-reload
systemctl --user enable lydia-update.timer

exec systemctl --user restart lydia.service
