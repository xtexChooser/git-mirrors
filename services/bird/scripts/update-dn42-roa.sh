#!/usr/bin/env bash
set -e

curl -SL -o /etc/bird/roa_dn42.conf 'https://explorer.burble.com/api/roa/bird/2/4'
curl -SL -o /etc/bird/roa_dn42_v6.conf 'https://explorer.burble.com/api/roa/bird/2/6'
"$(dirname "$0")/reconf.sh"
