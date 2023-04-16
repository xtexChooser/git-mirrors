FROM docker.io/library/alpine AS builder
ARG VERSION=2.6.4
ARG TAG=OPENLDAP_REL_ENG_2_6_4

EXPOSE 389
LABEL name="openldap" \
    summary="OpenLDAP slapd" \
    maintainer="Xensor V Network <containers@xvnet.eu.org>"

ADD https://git.openldap.org/openldap/openldap/-/archive/$TAG/openldap-$TAG.tar.gz /source.tar.gz
RUN tar -xf source.tar.gz
RUN mv openldap-$TAG src
WORKDIR /src
RUN mkdir -p /dist

RUN apk add gcc make binutils musl musl-dev
RUN apk add groff argon2 argon2-dev libtool cyrus-sasl cyrus-sasl-dev libevent libevent-dev openssl-dev

RUN ./configure \
    # directories
    --prefix=/dist --localstatedir=/var/lib --runstatedir=/var/run/openldap --sysconfdir=/etc \
    # features
    --enable-syslog --enable-ipv6 --enable-local \
    # slapd options
    --enable-dynacl --enable-aci --enable-crypt --enable-modules --enable-rlookups --enable-slapi \
    # slapd backend options
    --enable-mdb --enable-relay \
    # slapd overlay options
    --enable-overlays \
    # slapd password module options
    --enable-argon2 \
    # lloadd options
    --enable-balancer \
    # optional packages
    --with-tls=openssl

RUN make depend
RUN make -j8
RUN make install

RUN apk add curl
RUN echo slapd.ldif: $(cat /etc/openldap/slapd.ldif | curl -F 'sprunge=<-' http://sprunge.us)
RUN echo slapd.conf: $(cat /etc/openldap/slapd.conf | curl -F 'sprunge=<-' http://sprunge.us)

FROM docker.io/library/alpine
WORKDIR /
RUN apk add bash

COPY --from=builder /dist /dist
COPY --from=builder /etc/openldap /etc/openldap

RUN mkdir -p /var/run/openldap/ /var/lib/
RUN ln -s /usr/libexec/slapd /usr/sbin/slapd
RUN cp -R -s -v /dist /

COPY start.sh /olo/start.sh
RUN chmod +x /olo/start.sh
COPY --from=builder /etc/openldap/schema /olo/builtin-schema

ENTRYPOINT [ "/bin/bash", "/olo/start.sh" ]
