#!/usr/bin/env bash

set -e
pkg="$1"

apk add podman buildah

podman login codeberg.org -p "$codeberg_token" -u xtex

cd "$pkg"/
../xcbuild build
../xcbuild publish
