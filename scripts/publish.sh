#! /usr/bin/bash

version=$(grep '^version = "' pack.toml | sed -e 's/version = "//' | sed -e 's/"//')

rm -- *.mrpack *.zip

scripts/update-credits.sh
packwiz refresh

git add .
git commit -S -s -m "release: $version"
