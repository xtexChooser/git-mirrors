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

echo load tf jobs
toolforge-jobs load ./d/jobs.yaml
toolforge-jobs list

echo build worker
source .profile
cargo -V
toolforge-jobs run build --command d/build.sh --image bullseye --emails=onfailure --wait
cat build.out
cat build.err
rm build.out build.err

echo restart webservice
webservice restart
webservice status
