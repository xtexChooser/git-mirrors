include:
    - container
    - caddy
    - spica

{% set certs = ["A1"] %}

spica-signer:
    file.managed:
        - name: /etc/spica/signer/spica-signer.toml
        - source: salt://spica/signer/spica-signer.toml.j2
        - context:
            tpldir: spica/
        - template: jinja
        - user: root
        - group: root
        - mode: "0644"
        - makedirs: True
        - show_changes: False
        - require:
{% for cert in certs %}
            - file: spica-signer-{{ cert }}
{% endfor %}
    docker_container.running:
        - image: codeberg.org/xvnet/spica:latest
        - cmd: spica-signer
        - require:
            - test: container
            - docker_image: spica
            - file: spica-signer
            - docker_network: caddy
        - networks:
            - caddy:
                - aliases: []
        - hostname: spica-signer
        - environment:
            - HOME=/root
        - watch:
            - file: spica-signer
        - binds:
            - /etc/spica/signer:/etc/spica/signer
        - memory: 32M

{% for cert in certs %}
spica-signer-{{ cert }}:
    file.managed:
        - name: /etc/spica/signer/{{ cert }}.pem
        - source: salt://spica/certs/{{ cert }}.pem
        - template: null
        - user: root
        - group: root
        - mode: "0600"
        - makedirs: True
        - show_changes: False
{% endfor %}
