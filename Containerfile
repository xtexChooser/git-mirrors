FROM docker.io/library/alpine AS bld

RUN apk add --no-cache bash php composer

ADD mw /mw
ADD strip.sh /

WORKDIR /mw
RUN composer install --ignore-platform-reqs --no-dev
RUN /strip.sh

FROM scratch
COPY --from=bld /mw /
