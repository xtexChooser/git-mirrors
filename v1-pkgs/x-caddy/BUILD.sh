# shellcheck shell=bash disable=SC2154,SC2034

pkgname=x-caddy
pkgver=0
url="https://github.com/caddyserver/caddy"
license="Apache-2.0"
ocibaseimage="docker.io/library/alpine:$alpinever"
ocibuilderimage="docker.io/library/alpine:$alpinever"

build() {
	xbuildoci
}

buildoci() {
	oci::from

	oci::builder
	oci::cd /build
	oci::run apk add go
	oci::env GOBIN /build
	oci::run go install github.com/caddyserver/xcaddy/cmd/xcaddy@"$xcaddyver"
	oci::run /build/xcaddy build \
		--with github.com/mholt/caddy-ratelimit \
		--with github.com/ueffel/caddy-brotli \
		--with github.com/caddyserver/replace-response \
		--with github.com/mholt/caddy-l4 \
		--with github.com/mholt/caddy-events-exec
	oci::cpbld /build/caddy /usr/bin/caddy
	oci::endbld

	oci::entrypoint '[ "/usr/bin/caddy", "run", "--environ", "--config", "/etc/caddy/Caddyfile" ]'

	oci::end
}

publish() {
	xpuboci
}
