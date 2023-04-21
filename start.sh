#!/bin/bash

SLAPD_CONF_PATH=${SLAPD_CONF_PATH:-/etc/openldap/slapd.conf}
SLAPD_D_PATH=${SLAPD_D_PATH:-/tmp/slapd.d}
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

if [[ "x${OLO_NO_LN_BUILTIN_SCHEMA:-true}" != "xtrue" ]]; then
    echo copying builtin schemas
    # shellcheck disable=SC2046
    cp -R -s -n $(readlink -f "${OLO_BUILTIN_SCHEMA_PATH:-/olo/schema}")/* "$(readlink -f "${OLO_SCHEMA_LN_DST:-/etc/openldap/schema/}")"
fi

if [[ "x$OLO_NO_IMPORT_SLAPD_LDIF" != "xtrue" ]]; then
if [[ -e "$SLAPD_LDIF_PATH" ]]; then
    echo importing slapd.ldif
    mkdir "$SLAPD_D_PATH"
    /usr/sbin/slapadd -n 0 -F "$SLAPD_D_PATH" -c -l "$SLAPD_LDIF_PATH"
    SLAPD_OPTS+=("-F" "$SLAPD_D_PATH")
fi
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