#!/usr/bin/env bash
set -euo pipefail

version="$(grep -E '^version = "(.*)"$' Cargo.toml | head -n1 | tail -c+12 | head -c-2)"
[[ -e Cargo.toml ]] || exit 1
mkdir -p maint/dist

echo "Creating release ..."
if [[ "${NOREL:-}" != "" ]]; then
    echo "Skipping releasing ..."
else
    cargo release patch -x --no-confirm
fi

echo "Setting version to $version ..."
yq -i -I 4 ".version |= \"$version\"" maint/version.json

echo "Building x86-64 prebuilt binary ..."
cargo build --release --target x86_64-pc-windows-gnu

echo "Compressing binaries with UPX ..."
rm -vf maint/dist/yzt-prebuilt.exe
upx --best --lzma target/x86_64-pc-windows-gnu/release/yjyz-tools.exe \
    -o maint/dist/yzt-prebuilt.exe

echo "Updating checksums ..."
checksum="$(sha256sum maint/dist/yzt-prebuilt.exe | cut -d' ' -f1)"
yq -i -I 4 ".sha256sum |= \"$checksum\"" maint/version.json

echo "Preparing version JSON ..."
cp -vp maint/version.json maint/dist/version_v1.json
yq -i -I 0 "." maint/dist/version_v1.json

echo "Sending files ..."
rsync -rvp maint/dist/ envs.net:public_html/yjyz-tools/

echo "New version published!"
