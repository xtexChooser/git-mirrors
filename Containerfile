FROM docker.io/library/caddy:builder AS builder

RUN xcaddy build \
    --with github.com/caddyserver/replace-response \
    --with github.com/hairyhenderson/caddy-teapot-module \
	--with github.com/caddyserver/cache-handler
#	--with github.com/mholt/caddy-ratelimit

FROM docker.io/library/caddy:latest

COPY --from=builder /usr/bin/caddy /usr/bin/caddy

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
