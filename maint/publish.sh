#!/usr/bin/env bash
set -euo pipefail

version="$(grep -E '^version = "(.*)"$' Cargo.toml | head -n1 | tail -c+12 | head -c-2)"

echo "Setting version to $version ..."
yq -i -I 4 ".version |= \"$version\"" maint/version.json

echo "Building x86-64 prebuilt binary ..."
cargo build --release --target x86_64-pc-windows-gnu

echo "Updating checksums ..."
checksum="$(sha256sum target/x86_64-pc-windows-gnu/release/yjyz-tools.exe | cut -d' ' -f1)"
yq -i -I 4 ".sha256sum |= \"$checksum\"" maint/version.json

echo "Sending files ..."
rsync -p maint/version.json \
    envs.net:public_html/yjyz-tools/version_v1.json
rsync -p target/x86_64-pc-windows-gnu/release/yjyz-tools.exe \
    envs.net:public_html/yjyz-tools/yzt-prebuilt.exe
