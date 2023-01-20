FROM docker.io/library/alpine AS builder
ARG VERSION=v2.0.11

RUN apk add clang make autoconf binutils musl musl-dev gcc
RUN apk add flex-dev bison m4 libssh-dev linux-headers ncurses-dev readline-dev git
ADD https://gitlab.nic.cz/labs/bird/-/archive/$VERSION/bird-$VERSION.tar.gz /source.tar.gz
RUN tar -xf source.tar.gz
RUN mv bird-$VERSION src
WORKDIR /src
RUN mkdir -p /dist
ENV CC=clang
RUN autoreconf
RUN ./configure --prefix=/dist --sysconfdir=/etc/bird --runstatedir=/var/run/
RUN make
RUN make install

FROM docker.io/library/alpine
RUN apk add libssh
COPY --from=builder /dist /
RUN mkdir -p /var/run/
WORKDIR /
ENTRYPOINT [ "/sbin/bird" ]
#VOLUME [ "/var/run/", "/etc/bird/" ]
