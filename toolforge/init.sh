#!/usr/bin/bash
#
# Configure a new lydia installtion on Toolforge:
# curl -Ls https://codeberg.org/Lydia/deployment/raw/branch/main/toolforge/init.sh | bash
#

set -xe
echo "[init.sh]"

echo clone deployment.git into ./deployment
git clone https://codeberg.org/Lydia/deployment.git deployment
cd deployment; git describe --all --long; cd ..

echo link ./d to ./deployment/toolforge
ln -s ./deployment/toolforge ./d

echo clone lydia.git into ./src
git clone https://codeberg.org/Lydia/lydia.git src
cd src; git describe --all --long; cd ..

echo init static git repo mirror
mkdir -p ~/www/static/source
ln -s ~/src/.git ~/www/static/source/.git
ln -s ~/src/.git ~/www/static/source.git
cd ~/www/static/source.git
git update-server-info
mv hooks/post-update.sample hooks/post-update
ln -s hooks/post-update hooks/post-commit
ln -s hooks/post-update hooks/post-rewrite
ln -s hooks/post-update hooks/post-update
chmod a+x hooks/post-update

echo configure webservice
ln -s ~/deployment/toolforge/service.template service.template

echo configure rustup
tee ~/.profile << EOF
. "/data/project/rustup/rustup/.cargo/env"
export RUSTUP_HOME=/data/project/rustup/rustup/.rustup
EOF
source "$HOME/.profile"

LYDIA_FORCE_UPDATE=true exec ~/deployment/toolforge/update.sh
