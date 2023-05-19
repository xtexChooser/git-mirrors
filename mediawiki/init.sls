include:
    - container
    - caddy
    - mediawiki.src

extend:
    caddy:
        docker_container:
            - binds:
                - mediawiki-src:/srv/mw/w:ro

mediawiki:
    docker_container.running:
        - image: codeberg.org/xvnet/mediawiki:latest
        - binds:
            - mediawiki-src:/srv/mw/w:ro
            - /srv/mw/images:/srv/mw/images:rw
            - /srv/mw/config/LocalSettings.php:/srv/mw/config/LocalSettings.php:ro
        - require:
            - test: container
            - file: /srv/mw/images
            - file: /srv/mw/config/LocalSettings.php
        - memory: 64M
        - hostname: mediawiki
        - environment:
            - HOME: /root
        - networks:
            - caddy:
                - aliases: []

/srv/mw/images:
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

/srv/mw/config/LocalSettings.php:
    file.managed:
        - source: salt://mediawiki/LocalSettings.php
        - template: jinja
        - user: root
        - group: root
        - mode: "0644"
        - require:
            - file: /srv/mw/images
