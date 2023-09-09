FROM docker.io/library/alpine:latest AS builder

RUN apk add --no-cache make pandoc jq jo

COPY . /src/
WORKDIR /src
RUN make -j4

FROM docker.io/library/caddy:latest

RUN apk add --no-cache bash

RUN caddy add-package \
	github.com/caddyserver/replace-response \
    github.com/hairyhenderson/caddy-teapot-module \
	github.com/aksdb/caddy-cgi/v2
#	github.com/caddyserver/cache-handler
#	github.com/mholt/caddy-ratelimit

LABEL org.opencontainers.image.title="xtex's Home"
LABEL org.opencontainers.image.description="xtex's Home Directory"
LABEL org.opencontainers.image.url=https://xtexx.eu.org
LABEL org.opencontainers.image.vendor=xtex
LABEL org.opencontainers.image.licenses=MPL-2.0
LABEL org.opencontainers.image.source="https://codeberg.org/xtex/home"

COPY Caddyfile /etc/caddy/Caddyfile
COPY --from=builder /src/src /srv/src
RUN mkdir /srv/run

ENV PROD true
ENV UDS_DIR_ADMIN run
ENV UDS_DIR run
