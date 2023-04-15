FROM docker.io/library/alpine AS builder
ARG VERSION=2.6.4
ARG TAG=OPENLDAP_REL_ENG_2_6_4

LABEL name="openldap" \
    summary="OpenLDAP slapd" \
    maintainer="Xensor V Network <containers@xvnet.eu.org>"

ADD https://git.openldap.org/openldap/openldap/-/archive/$TAG/openldap-$TAG.tar.gz /source.tar.gz
RUN tar -xf source.tar.gz
RUN mv openldap-$TAG src
WORKDIR /src
RUN mkdir -p /dist

RUN apk add gcc make binutils musl musl-dev groff

RUN ./configure --prefix=/dist --localstatedir=/var --sysconfdir=/etc\
    --enable-syslog --enable-ipv6 --with-tls --enable-local 
RUN make depend
RUN make -j8
RUN make install

FROM docker.io/library/alpine
#RUN apk add libssh ncurses readline
COPY --from=builder /dist /usr
COPY --from=builder /etc/openldap /etc/openldap
RUN mkdir -p /var/run/slapd/ /var/lib/
RUN ln -s /usr/libexec/slapd /usr/sbin/slapd
WORKDIR /
ENTRYPOINT [ "/libexec/slapd", "-F", "/etc/openldap/slapd.d", "-h", "ldap:/// ldaps:/// ldapi:///" ]
