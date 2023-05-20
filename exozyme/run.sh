#!/usr/bin/bash
#
# Run entrypoint
#

set -xe
echo "[run.sh]"

cd ~/lydia/src
pwd

echo make sure lydia has been built
cargo build --package lydia-worker --release

echo starting lydia
sha256sum ./target/release/lydia-worker
exec ./target/release/lydia-worker
