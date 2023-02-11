{% if grains.os_family == 'Debian' or grains.os_family == 'FreeBSD' %}
  {% set bird_pkg = 'bird2' %}
{% else %}
  {% set bird_pkg = 'bird' %}
{% endif %}

bird:
  file.managed:
    - name: /etc/bird/bird.conf
    - source: salt://bird/bird.conf.j2
    - context:
        tpldir: bird/
    - template: jinja
    - user: bird
    - group: bird
    - mode: "0666"
  docker_image.present:
    - name: ghcr.io/xtex-vnet/bird
    - tag: {{ pillar['network']['routing']['bird_version'] }}
  docker_container.running:
    - image: ghcr.io/xtex-vnet/bird:{{ pillar['network']['routing']['bird_version'] }}
    - binds:
      - /etc/bird:/etc/bird:ro
      - /var/run/bird:/var/run/bird:rw
    - publish_all_ports: True
    - network_mode: host
    - ipc_mode: host
    - cap_add: CAP_NET_ADMIN
    - require:
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

remove old BIRD images:
  docker_image.absent:
    - images:
      - ghcr.io/xtex-vnet/bird:2.0.11-7
