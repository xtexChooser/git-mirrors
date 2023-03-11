{% if grains.os_family == 'Debian' or grains.os_family == 'FreeBSD' %}
  {% set bird_pkg = 'bird2' %}
{% else %}
  {% set bird_pkg = 'bird' %}
{% endif -%}
{% set BIRD_VERSION = pillar['network']['routing']['bird_version'] -%}

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
    - tag: {{ BIRD_VERSION }}
    - require:
      - test: container
  docker_container.running:
    - image: ghcr.io/xtex-vnet/bird:{{ BIRD_VERSION }}
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
      - service: bird
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
  pkg.latest:
    - name: {{ bird_pkg }}
  service.dead:
    - enable: False
    - require:
      - pkg: bird
