include:
    - container
    - caddy

mediawiki:
    docker_container.running:
        - image: codeberg.org/xvnet/mediawiki:latest
        - binds:
            - /var/www/mw/images:/var/www/html/images
            - /var/www/mw/LocalSettings.php:/var/www/html/LocalSettings.php
        - require:
            - test: container
            - file: /var/www/mw/images
            - file: /var/www/mw/LocalSettings.php
        - memory: 64M
        - hostname: mediawiki
        - environment:
            - HOME: /root
        - networks:
            - caddy:
                - aliases: []

/var/www/mw/images:
    file.directory:
        - user: root
        - group: root
        - dir_mode: 660
        - makedirs: true

/etc/caddy/sites/mediawiki.conf:
    file.managed:
        - source: salt://mediawiki/Caddyfile.j2
        - template: jinja
        - user: root
        - group: root
        - mode: "0644"
        - require:
            - file: /etc/caddy/sites

/var/www/mw/LocalSettings.php:
    file.managed:
        - source: salt://mediawiki/LocalSettings.php
        - template: jinja
        - user: root
        - group: root
        - mode: "0600"
        - require:
            - file: /var/www/mw/images
