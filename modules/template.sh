#!/usr/bin/env bash

set -e
tmpfile="$(mktemp)"
make do-tpl TPL_BACKEND="$1" TPL_IN="$2" TPL_OUT="$tmpfile" &>/dev/stderr
cat "$tmpfile"
rm -f "$tmpfile" &>/dev/stderr
