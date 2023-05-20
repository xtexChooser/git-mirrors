#!/usr/bin/bash

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

exec ./deployment/exozyme/update.sh
