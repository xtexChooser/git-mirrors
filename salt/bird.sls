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
  service.running:
    - enable: True
    - reload: True
    - watch:
      - file: bird
