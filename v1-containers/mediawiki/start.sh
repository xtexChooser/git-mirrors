#!/bin/bash

cp -R -f /etc/mw/* "$(pwd)/"

/usr/local/sbin/php-fpm
