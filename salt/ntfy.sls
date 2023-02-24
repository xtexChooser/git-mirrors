{% set NTFY_VERSION = "v2.0.1" -%}

ntfy:
  file.managed:
    - name: /etc/ntfy/server.yml
    - source: salt://ntfy/server.yml.j2
    - context:
        tpldir: ntfy/
    - template: jinja
    - user: root
    - group: root
    - mode: "0644"
    - makedirs: True
  docker_image.present:
    - name: docker.io/binwiederhier/ntfy
    - tag: {{ NTFY_VERSION }}
    - require:
      - test: container
  docker_container.running:
    - image: docker.io/binwiederhier/ntfy:{{ NTFY_VERSION }}
    - binds:
      - /etc/ntfy:/etc/ntfy
      - /var/cache/ntfy:/var/cache/ntfy
      - /var/run/ntfy:/var/run/ntfy
      - /var/lib/ntfy:/var/lib/ntfy
    - cmd: serve
    - require:
      - test: container
      - docker_image: ntfy
      - file: ntfy
    - environment:
      - HOME=/root
      - HOSTNAME={{ grains['fqdn'] }}
    - watch:
      - file: ntfy
