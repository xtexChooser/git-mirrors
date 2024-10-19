#!/usr/bin/env bash

set -e

wiki=$1
shift
exec podman exec -it mediawiki php maintenance/run.php shell --wiki "$wiki"
