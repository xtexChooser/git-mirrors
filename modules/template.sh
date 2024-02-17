#!/usr/bin/env bash

set -e
tmpfile="$(mktemp)"
make do-tpl TPL_BACKEND="$1" TPL_IN="$2" TPL_OUT="$tmpfile" >&2
cat "$tmpfile"
rm -f "$tmpfile"
