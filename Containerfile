FROM docker.io/library/alpine AS builder
ARG VERSION

RUN apk add clang make autoconf binutils musl musl-dev gcc sudo libcap
RUN apk add flex-dev bison m4 libssh-dev linux-headers ncurses-dev readline-dev git
ADD https://gitlab.nic.cz/labs/bird/-/archive/v$VERSION/bird-v$VERSION.tar.gz /source.tar.gz
RUN tar -xf source.tar.gz
RUN mv bird-v$VERSION src
WORKDIR /src
RUN mkdir -p /dist
ENV CC=clang
RUN autoreconf
RUN ./configure --prefix=/dist --sysconfdir=/etc/bird --runstatedir=/var/run/bird
RUN make
RUN make install
RUN sudo setcap CAP_NET_ADMIN=+eip /dist/sbin/bird

FROM docker.io/library/alpine
RUN apk add libssh ncurses readline
COPY --from=builder /dist /
RUN mkdir -p /var/run/
WORKDIR /
ENTRYPOINT [ "/sbin/bird", "-f" ]
#VOLUME [ "/var/run/", "/etc/bird/" ]
