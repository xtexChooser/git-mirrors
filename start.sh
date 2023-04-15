#!/bin/bash

SLAPD_D_PATH=${SLAPD_D_PATH:-/etc/openldap/slapd.d}
SLAPD_CONF_PATH=${SLAPD_CONF_PATH:-/etc/openldap/slapd.conf}
SLAPD_LDIF_PATH=${SLAPD_LDIF_PATH:-/etc/openldap/slapd.ldif}

SLAPD_LISTEN=${SLAPD_LISTEN:-ldap:/// ldaps:/// ldapi:///}
SLAPD_OPTS=${SLAPD_OPTS:-}

if [[ "x$OLO_NO_DEFAULT_F" != "xtrue" ]]; then
    echo adding default configuration paths
    SLAPD_OPTS=$SLAPD_OPTS "-F" "$SLAPD_D_PATH" "-f" "$SLAPD_CONF_PATH"
    if [[ ! -e "$SLAPD_D_PATH" ]]; then
        mkdir -v -p "$SLAPD_D_PATH"
    fi
fi

if [[ "x$OLO_NO_DEFAULT_H" != "xtrue" ]]; then
    echo adding default listen addrs
    SLAPD_OPTS=$SLAPD_OPTS "-h" "$SLAPD_LISTEN"
fi

if [[ "x$OLO_NO_LN_BUILTIN_SCHEMA" != "xtrue" ]]; then
    echo copying builtin schemas
    cp -R -s -n "$(readlink -e "${OLO_BUILTIN_SCHEMA_PATH:-/olo/builtin-schema}")" "$(readlink -m "${OLO_SCHEMA_LN_DST:-/etc/openldap/schema}")"
fi

if [[ "x$OLO_NO_IMPORT_SLAPD_LDIF" != "xtrue" ]]; then
    echo importing slapd.ldif
    /usr/sbin/slapadd -n 0 -c -F "$SLAPD_D_PATH" -l "$SLAPD_LDIF_PATH"
fi

# shellcheck disable=SC2086
echo executing slapd with $SLAPD_OPTS
# shellcheck disable=SC2086
exec "/usr/bin/slapd" $SLAPD_OPTS
