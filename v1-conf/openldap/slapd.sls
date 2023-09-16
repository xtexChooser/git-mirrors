{% set VERSION = '2.6.4' %}

include:
    - container

slapd:
    docker_container.running:
        - image: codeberg.org/xvnet/openldap:{{ VERSION }}
        - binds:
            - /etc/openldap:/etc/openldap:ro
            - /var/lib/openldap:/var/openldap-data
            - /opt:/opt:ro
        - require:
            - test: container
            - file: slapd.conf
            - file: /var/lib/openldap
        - watch:
            - file: slapd.conf
        - memory: 128M
        - hostname: slapd
        - environment:
            - HOME=/root
        - port_bindings:
            - 389:389
            - 636:636

slapd.conf:
    file.managed:
        - name: /etc/openldap/slapd.conf
        - source: salt://openldap/slapd.conf.j2
        - context:
            tpldir: openldap/
        - template: jinja
        - user: root
        - group: root
        - mode: "0600"
        - makedirs: True

/var/lib/openldap:
    file.directory:
        - user: root
        - group: root
        - dir_mode: 700
