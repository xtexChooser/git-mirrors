#!/usr/bin/env sh

repo="$1"
version="$(grep -F '# TAG:LATEST' <"$2" | cut -d'-' -f2 | cut -d'#' -f1 | xargs)"
pkgversion="$3"
[ "$version" == "$pkgversion" ] && {
	echo Logging into registry
	skopeo login -u "$CODEBERG_TOKEN" -p "$CODEBERG_TOKEN" codeberg.org
	echo Copying to latest
	skopeo copy -a "docker://$repo:$pkgversion" "docker://$repo:latest"
}
