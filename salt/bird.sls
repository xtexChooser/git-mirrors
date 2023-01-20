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
  service.dead:
    - enable: False
#    - reload: True
#    - watch:
#      - file: bird
  docker_image.present:
    - name: ghcr.io/xtex-vnet/bird
    - tag: {{ pillar['network']['routing']['bird_version'] }}
  docker_container.running:
    - image: ghcr.io/xtex-vnet/bird:{{ pillar['network']['routing']['bird_version'] }}
    - binds:
      - /etc/bird:/etc/bird:ro
      - /var/run/bird:/var/run/bird:rw
    - publish_all_ports: True
    - require:
      - docker_image: bird
