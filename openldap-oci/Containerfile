FROM docker.io/library/alpine AS builder
ARG VERSION=2.6.4

EXPOSE 389
LABEL name="openldap" \
    summary="OpenLDAP slapd" \
    maintainer="Xensor V Network <containers@xvnet.eu.org>"

RUN VERSION=$VERSION \
    && apk add curl \
    && TAG=OPENLDAP_REL_ENG_${VERSION//\./_} \
    && curl -o /source.tar.gz https://git.openldap.org/openldap/openldap/-/archive/$TAG/openldap-$TAG.tar.gz \
    && tar -xf source.tar.gz \
    && mv openldap-$TAG src
WORKDIR /src
RUN mkdir -p /dist

RUN apk add gcc make binutils musl musl-dev
RUN apk add groff argon2 argon2-dev libtool cyrus-sasl cyrus-sasl-dev libevent libevent-dev openssl-dev

RUN ./configure \
    # directories
    --prefix=/dist \
    --localstatedir=/var \
    --runstatedir=/var/run/openldap \
    --sysconfdir=/etc \
    # features
    --enable-syslog \
    --enable-ipv6=yes \
    --enable-local \
    # slapd options
    --enable-dynacl \
    --enable-aci \
    --enable-modules \
    --enable-shared \
    --enable-dynamic \
    --enable-crypt \
    --enable-wrappers=no \
    --enable-spasswd \
    # slapd backend options
    --enable-ldap=mod \
    --enable-mdb=mod \
    --enable-relay=mod \
    # slapd overlay options
    --enable-overlays=mod \
    --enable-syncprov=mod \
    --enable-ppolicy=mod \
    # slapd password module options
    --enable-argon2 \
    # lloadd options
    --enable-balancer \
    # others
    --with-cyrus-sasl \
    --with-tls=openssl \
    --with-yielding-select \
    --with-argon2=libargon2

RUN make depend
RUN make -j8
RUN make install

RUN apk add curl
RUN echo slapd.ldif: $(cat /etc/openldap/slapd.ldif | curl -F 'sprunge=<-' http://sprunge.us)
RUN echo slapd.conf: $(cat /etc/openldap/slapd.conf | curl -F 'sprunge=<-' http://sprunge.us)

FROM docker.io/library/alpine
WORKDIR /
# coreutils for GNU tail
RUN apk add bash coreutils
RUN apk add libtool cyrus-sasl

COPY --from=builder /dist /dist
COPY --from=builder /etc/openldap /etc/openldap

RUN mkdir -p /var/run/openldap/ /var/lib/ /var/openldap-data/
RUN cp -R -s -v /dist/* /usr/
RUN ln -s /usr/libexec/slapd /usr/sbin/slapd

COPY start.sh /olo/start.sh
RUN chmod +x /olo/start.sh
COPY --from=builder /etc/openldap/schema /olo/schema
RUN ls -R /olo/schema

ENTRYPOINT [ "/bin/bash", "/olo/start.sh" ]
