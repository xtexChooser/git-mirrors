#!/bin/bash

SLAPD_CONF_PATH=${SLAPD_CONF_PATH:-/etc/openldap/slapd.conf}
SLAPD_LDIF_PATH=${SLAPD_LDIF_PATH:-/etc/openldap/slapd.ldif}

SLAPD_LISTEN=${SLAPD_LISTEN:-ldap:/// ldaps:/// ldapi:///}
SLAPD_OPTS=${SLAPD_OPTS:-/usr/sbin/slapd}

if [[ "x$OLO_NO_DEFAULT_F" != "xtrue" ]]; then
    echo adding default configuration paths
    SLAPD_OPTS+=("-f" "$SLAPD_CONF_PATH")
fi

if [[ "x$OLO_NO_DEFAULT_H" != "xtrue" ]]; then
    echo adding default listen addrs
    SLAPD_OPTS+=("-h" "\"$SLAPD_LISTEN\"")
fi

if [[ "x$OLO_NO_LN_BUILTIN_SCHEMA" != "xtrue" ]]; then
    echo copying builtin schemas
    cp -R -s -n "$(readlink -f "${OLO_BUILTIN_SCHEMA_PATH:-/olo/builtin-schema}")/*" "$(readlink -f "${OLO_SCHEMA_LN_DST:-/etc/openldap/schema/}")"
fi

if [[ "x$OLO_NO_IMPORT_SLAPD_LDIF" != "xtrue" ]]; then
    echo importing slapd.ldif
    /usr/sbin/slapadd -n 0 -c -l "$SLAPD_LDIF_PATH"
fi

if [[ "x$OLO_NO_AUTO_USER" != "xtrue" ]]; then
    SLAPD_OPTS+=("-u" "$(id -n -u)" "-g" "$(id -n -g)")
fi

if [[ "x$OLO_NO_CHANGE_ULIMIT" != "xtrue" ]]; then
    ulimit -n 1024
fi

# shellcheck disable=SC2068
echo ${SLAPD_OPTS[@]}

# shellcheck disable=SC2068
bash -c ${SLAPD_OPTS[@]}

pid=$(cat /var/run/slapd.pid)
echo slapd pid: "$pid"
tail "--pid=$pid" -f /dev/null

if [[ -e /proc/$pid/cmdline ]]; then
    kill "$pid"
fi