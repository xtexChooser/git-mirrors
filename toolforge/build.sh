#!/usr/bin/bash
#
# Build script
#

set -xe
echo "[build.sh]"
pwd
source .profile

cd src
cargo -V
cargo build --package lydia-worker --release
