#!/usr/bin/env bash

BUILD_DATE=$(date)
VERSION=$1
MIGRATIONS=$2

function require() {
    if [ "$1" = "" ]; then
        echo "input '$2' required"
        print_help
        exit 1
    fi
}

function print_help() {
    echo "build.sh"
    echo ""
    echo "Usage:"
    echo "      build.sh [version]"
    echo ""
    echo "Args:"
    echo "      version: The version of the current container"
}

require "$VERSION" "version"

if ! docker run --rm -it arm64v8/ubuntu:19.10 /bin/bash -c 'echo "docker is configured correctly"'; then
    echo "docker is not configured to run on qemu-emulated architectures, fixing will require sudo"
    sudo docker run --rm --privileged multiarch/qemu-user-static --reset -p yes
fi

set -xe

cargo clean
cargo doc --no-deps

mkdir -p artifacts
rm -rf artifacts/html
cp -r ./target/doc artifacts/html

docker build \
    --pull \
    --no-cache \
    --build-arg BUILD_DATE="${BUILD_DATE}" \
    --build-arg TAG="${TAG}" \
    -f Dockerfile.arm64v8 \
    -t "asonix/activitystreams-ext-docs:${VERSION}-arm64v8" \
    -t "asonix/activitystreams-ext-docs:latest-arm64v8" \
    -t "asonix/activitystreams-ext-docs:latest" \
    ./artifacts

docker push "asonix/activitystreams-ext-docs:${VERSION}-arm64v8"
docker push "asonix/activitystreams-ext-docs:latest-arm64v8"
docker push "asonix/activitystreams-ext-docs:latest"
