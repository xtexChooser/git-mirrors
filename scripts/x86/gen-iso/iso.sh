#!/usr/bin/env bash
set -e

outdev="$1"
optsfile="$2"

rm -rf "$outdev"
exec xorriso -outdev "$outdev" -options_from_file "$optsfile" -end 2>&1
