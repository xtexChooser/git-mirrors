#!/usr/bin/env bash

set -e

grep -F '[submodule ' mw/.gitmodules | grep -Eo '(extensions|skins)/[^"]*' | while read -r n; do
	[[ ! -e "mw/$n" ]] || continue
	n="${n/\// }"
	kind="$(cut -d' ' -f1 <<<"$n")"
	name="$(cut -d' ' -f2 <<<"$n")"
	./addext.sh "$kind" "$name" 2>>.addbundled.log || continue
	git push
done

echo "Added, errors:"

if [[ -e ".addbundled.log" ]]; then
	cat .addbundled.log >&2
	rm .addbundled.log
fi
