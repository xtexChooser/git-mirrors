FROM docker.io/library/alpine AS builder
ARG VERSION=2.3.2

EXPOSE 3389 3636
LABEL name="389ds-dirsrv" \
    summary="389 Directory Server" \
    maintainer="Xensor V Network <containers@xvnet.eu.org>"

ADD https://github.com/389ds/389-ds-base/archive/refs/tags/389-ds-base-$VERSION.tar.gz /source.tar.gz
RUN tar -xf source.tar.gz
RUN mv 389-ds-base-389-ds-base-$VERSION src
RUN rm source.tar.gz
WORKDIR /src
RUN mkdir -p /dist

RUN apk add rust cargo musl musl-dev
RUN apk add autoconf
RUN apk add cyrus-sasl krb5
# db48-utils libLLVM7 libedit0 libgit2-26 libhttp_parser2_7_1 libssh2-1 mozilla-nss-tools

RUN autoreconf -fiv
RUN ./configure \
    --program-prefix= \
    --disable-dependency-tracking \
    --prefix=/usr \
    --exec-prefix=/usr \
    --bindir=/usr/bin \
    --sbindir=/usr/sbin \
    --sysconfdir=/etc \
    --datadir=/usr/share \
    --includedir=/usr/include \
    --libdir=/usr/lib64 \
    --libexecdir=/usr/lib \
    --localstatedir=/var \
    --sharedstatedir=/var/lib \
    --mandir=/usr/share/man \
    --infodir=/usr/share/info \
    --disable-dependency-tracking \
    --enable-debug \
    --enable-gcc-security --enable-autobind --enable-auto-dn-suffix --with-openldap \
    --enable-cmocka --enable-rust --disable-perl --with-pythonexec="python3" --without-systemd \
    --libexecdir=/usr/lib/dirsrv/ --prefix=/
RUN make -j8
RUN make install
RUN make lib389
RUN make lib389-install
WORKDIR /
RUN rm -rf /src

RUN mkdir -p /data/config
RUN mkdir -p /data/ssca
RUN mkdir -p /data/run
RUN mkdir -p /var/run/dirsrv
RUN ln -s /data/config /etc/dirsrv/slapd-localhost
RUN ln -s /data/ssca /etc/dirsrv/ssca
RUN ln -s /data/run /var/run/dirsrv

HEALTHCHECK --start-period=5m --timeout=5s --interval=30s --retries=2 \
    CMD /usr/libexec/dirsrv/dscontainer -H

ENTRYPOINT [ "/usr/libexec/dirsrv/dscontainer", "-r" ]
