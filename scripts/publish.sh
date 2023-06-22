#! /usr/bin/bash

version=$(grep '^version = "' pack.toml | sed -e 's/version = "//' | sed -e 's/"//')
if git tag | grep -q "$version"; then
    echo "$version" is already tagged
    exit
fi
changelog=$(git log "$(git tag | head -n1)"..HEAD --oneline --decorate=no --abbrev)

rm -q -- *.mrpack *.zip

scripts/update-credits.sh
packwiz refresh

git commit -uall -a -S -s --allow-empty -m "release: $version\n\n$changelog"
git tag -S "$version"
git push
git push --tags

scripts/sync-readme.sh
CHANGELOG="$changelog" scripts/upload-to-mr.sh
