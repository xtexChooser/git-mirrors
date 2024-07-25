FROM docker-registry.wikimedia.org/dev/buster-php81-fpm:1.0.1-s2
RUN curl -fsSL https://raw.githubusercontent.com/tj/n/master/bin/n | bash -s lts

