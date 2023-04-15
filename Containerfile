FROM docker.io/library/alpine AS builder
ARG VERSION=2.6.4
ARG TAG=OPENLDAP_REL_ENG_2_6_4

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
COPY --from=builder /dist /
COPY --from=builder /etc/openldap /etc/openldap
RUN mkdir -p /var/run/ /var/lib/
WORKDIR /
ENTRYPOINT [ "/libexec/slapd" ]
