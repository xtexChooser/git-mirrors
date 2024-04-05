#!/usr/bin/env bash

set -e

wiki=$1
shift
podman exec -it mediawiki php maintenance/run.php --wiki "$wiki" "$@"
