/opt/node_tls/agent_update:
    file.managed:
        - source: salt://node_tls/agent_update
        - template: jinja
        - user: root
        - group: root
        - mode: "0744"
        - makedirs: True

/opt/tls:
    file.directory:
        - user: root
        - group: root
        - dir_mode: 755
