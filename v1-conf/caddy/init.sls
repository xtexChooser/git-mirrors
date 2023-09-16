include:
    - container

caddy:
    file.managed:
        - name: /etc/caddy/Caddyfile
        - source: salt://caddy/Caddyfile.j2
        - context:
            tpldir: caddy/
        - template: jinja
        - user: root
        - group: root
        - mode: "0666"
        - makedirs: True
    docker_image.present:
        - name: codeberg.org/xvnet/caddy
        - tag: latest
        - require:
            - test: container
    docker_network.present:
        - driver: bridge
        - ipam_driver: default
        - ipam_opts: driver=host-local
    docker_container.running:
        - image: codeberg.org/xvnet/caddy:latest
        - binds:
            - /etc/caddy:/etc/caddy:ro
            - /var/run:/var/run
            - /var/lib/caddy:/root/.local/share/caddy
        - port_bindings:
            - 80:80
            - 443:443
        - cap_add: CAP_NET_BIND_SERVICE
        - networks:
            - caddy:
                - aliases: []
        - require:
            - test: container
            - docker_image: caddy
            - docker_network: caddy
            - file: caddy
            - file: /etc/caddy/sites
        - hostname: caddy
        - environment:
            - HOME=/root
        - watch:
            - file: caddy
        - memory: 32M

/etc/caddy/sites:
    file.directory:
        - user: root
        - group: root
        - dir_mode: 644
