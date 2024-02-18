# shellcheck shell=bash
# shellcheck disable=SC2154

case "${cleanups[@]}" in *"oci_cleanup"*) return ;; esac
cleanups+=('oci_cleanup')

declare -a ociwork
oci_cleanup() {
	for cont in "${ociwork[@]}"; do
		msg "Clean-up $cont"
		$BUILDAH rm "$cont"
	done
}

oci::from() {
	ociwork+=("$($BUILDAH from "$@" "$ocibaseimage")")

	oci::label org.opencontainers.image.title "$pkgname"
	oci::label org.opencontainers.image.description "$url"
	oci::label org.opencontainers.image.vendor "XV-NET"
	oci::label org.opencontainers.image.licenses "$license"
	oci::label org.opencontainers.image.source "https://codeberg.org/xvnet/containers"
}

oci::builder() {
	ociwork+=("$($BUILDAH from "$@" "$ocibuilderimage")")
}

oci::endbld() {
	$BUILDAH rm "${ociwork[-1]}"
	unset 'ociwork[-1]'
}

oci::run() {
	$BUILDAH run "${ociwork[-1]}" -- "$@"
}

oci::cfg() {
	$BUILDAH config "$@" "${ociwork[-1]}"
}

oci::label() {
	$BUILDAH config --label "$1"="$2" "${ociwork[-1]}"
}

oci::env() {
	$BUILDAH config --env "$1"="$2" "${ociwork[-1]}"
}

oci::entrypoint() {
	$BUILDAH config --entrypoint "$1" "${ociwork[-1]}"
}

oci::cd() {
	$BUILDAH config --workingdir "$1" "${ociwork[-1]}"
}

oci::cp() {
	$BUILDAH copy "${ociwork[-1]}" "$@"
}

oci::cpbld() {
	$BUILDAH copy --from "${ociwork[-1]}" "${ociwork[-2]}" "$@"
}

oci::add() {
	$BUILDAH add "${ociwork[-1]}" "$@"
}

oci::end() {
	$BUILDAH commit "$@" "${ociwork[-1]}" "$ociimage"
	$BUILDAH rm "${ociwork[-1]}"
	unset 'ociwork[-1]'
}
