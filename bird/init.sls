{% if grains.os_family == 'Debian' or grains.os_family == 'FreeBSD' %}
    {% set bird_pkg = 'bird2' %}
{% else %}
    {% set bird_pkg = 'bird' %}
{% endif -%}
{% set bird_version = pillar['network']['routing']['bird_version'] -%}

bird:
    file.managed:
        - name: /etc/bird/bird.conf
        - source: salt://bird/bird.conf.j2
        - context:
                tpldir: bird/
        - template: jinja
        - user: root
        - group: root
        - mode: "0655"
        - makedirs: True
        - require:
            - pkg: bird
    docker_image.present:
        - name: ghcr.io/xtex-vnet/bird
        - tag: {{ bird_version }}
        - require:
            - test: container
    docker_container.running:
        - image: ghcr.io/xtex-vnet/bird:{{ bird_version }}
        - binds:
            - /etc/bird:/etc/bird:ro
            - /var/run/bird:/var/run/bird:rw
        - publish_all_ports: True
        - network_mode: host
        - ipc_mode: host
        - cap_add: CAP_NET_ADMIN
        - require:
            - test: container
            - docker_image: bird
            - file: bird
        - environment:
            - HOME=/root
            - HOSTNAME={{ grains['fqdn'] }}
    cmd.run:
        - name: sudo birdc configure
        - onchanges:
            - file: bird
        - require:
            - docker_container: bird
            - pkg: bird
    pkg.removed:
        - name: {{ bird_pkg }}
