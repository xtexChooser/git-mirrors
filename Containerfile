FROM docker.io/library/caddy:latest

RUN caddy add-package \
	github.com/caddyserver/replace-response \
    github.com/hairyhenderson/caddy-teapot-module \
	github.com/caddyserver/cache-handler
#	github.com/mholt/caddy-ratelimit

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
