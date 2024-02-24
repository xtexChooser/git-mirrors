FROM docker.io/library/golang:alpine AS bld

WORKDIR /bld
RUN go install github.com/caddyserver/xcaddy/cmd/xcaddy@latest
RUN xcaddy build \
	--with github.com/caddyserver/replace-response \
	--with github.com/hairyhenderson/caddy-teapot-module \
	--with github.com/aksdb/caddy-cgi/v2

FROM docker.io/library/alpine:latest

COPY --from=bld /bld/caddy /usr/bin/caddy
RUN apk add --no-cache bash

LABEL org.opencontainers.image.title="xtex's Home"
LABEL org.opencontainers.image.description="xtex's Home Directory"
LABEL org.opencontainers.image.url=https://xtexx.eu.org
LABEL org.opencontainers.image.vendor=xtex
LABEL org.opencontainers.image.licenses=MPL-2.0
LABEL org.opencontainers.image.source="https://codeberg.org/xtex/home"

COPY Caddyfile /etc/caddy/Caddyfile
COPY src /srv/src
RUN mkdir /srv/run

ENV PROD true
ENV UDS_DIR_ADMIN run
ENV UDS_DIR run
ENV BLOG_DIR blog
