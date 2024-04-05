FROM codeberg.org/xvnet/x-mediawiki-php AS bld

RUN apk add --no-cache bash git
RUN php -r "copy('https://getcomposer.org/installer', 'composer-setup.php');"; \
	php -r "if (hash_file('sha384', 'composer-setup.php') === 'dac665fdc30fdd8ec78b38b9800061b4150413ff2e3b6f88543c636f7cd84f6db9189d43a81e5503cda447da73c7e5b6') { echo 'Installer verified'; } else { echo 'Installer corrupt'; unlink('composer-setup.php'); } echo PHP_EOL;"; \
	php composer-setup.php; \
	php -r "unlink('composer-setup.php');";

ADD mw /mw
ADD strip.sh /

WORKDIR /mw
RUN composer install --no-dev
RUN /strip.sh

FROM scratch
COPY --from=bld /mw /
