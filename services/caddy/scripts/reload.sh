#!/usr/bin/env bash
set -e
podman container exists caddy || exit

echo 'Reloading Caddy...'
sudo podman exec -it caddy caddy reload --config /etc/caddy/Caddyfile
echo 'Reloaded Caddy'
