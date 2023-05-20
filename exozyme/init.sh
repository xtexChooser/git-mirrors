#!/usr/bin/bash
#
# Configure a new lydia installtion on exozy.me:
# curl -Ls https://codeberg.org/Lydia/deployment/raw/branch/main/exozyme/init.sh | bash
#

set -xe
echo "[init.sh]"

mkdir ~/lydia/
cd ~/lydia/

echo clone deployment.git into ./deployment
git clone https://codeberg.org/Lydia/deployment.git deployment
cd deployment; git describe --all --long; cd ..

echo link ./d to ./deployment/exozyme
ln -s ./deployment/exozyme ./d

echo clone lydia.git into ./src
git clone https://codeberg.org/Lydia/lydia.git src
cd src; git describe --all --long; cd ..

echo create ~/.config/systemd/user
mkdir -p ~/.config/systemd/user

LYDIA_FORCE_UPDATE=true exec ./deployment/exozyme/update.sh
