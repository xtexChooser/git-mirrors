#!/usr/bin/env bash

set -e

[[ -e "contrib/generate.sh" ]] || cd ..

packages="$(echo */BUILD.sh | cut -d/ -f1)"

# Generate Woodpecker configs
rm -rf contrib/woodpecker/build-*.yaml
for pkg in $packages; do
	cat >>"contrib/woodpecker/build-$pkg.yaml" <<EOF
when:
    event: [push, tag, cron]
    path: "$pkg/*"

steps:
    build:
        group: buildpkg
        image: alpine
        secrets: [ codeberg_token ]
        commands:
            - apk add bash
            - ./contrib/woodpecker_build.sh $pkg

EOF
done
