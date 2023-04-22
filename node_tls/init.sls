include:
    - spica.signer

/root/node_tls/update:
    cron.present:
        - user: root
        - special: '@daily'
        - require:
            - file: /root/node_tls/update
    file.managed:
        - source: salt://node_tls/update
        - template: jinja
        - user: root
        - group: root
        - mode: "0744"
        - mkdirs: True