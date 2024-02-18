#!/usr/bin/env bash
set -e
exec &>>/var/log/dn42-roa-updater.log
curl -SL -o /etc/bird/roa_dn42.conf 'https://explorer.burble.com/api/roa/bird/2/4'
curl -SL -o /etc/bird/roa_dn42_v6.conf 'https://explorer.burble.com/api/roa/bird/2/6'
