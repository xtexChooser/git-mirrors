include:
    - container

mekbuda:
    file.managed:
        - name: /etc/mekbuda/mekbuda.toml
        - source: salt://mekbuda/mekbuda.toml.j2
        - context:
            tpldir: mekbuda/
        - template: jinja
        - user: root
        - group: root
        - mode: "0644"
        - makedirs: True
    docker_image.present:
        - name: codeberg.org/xvnet/mekbuda
        - tag: latest
        - require:
            - test: container
    docker_container.running:
        - image: codeberg.org/xvnet/mekbuda:latest
        - binds:
            - /etc/mekbuda:/etc/mekbuda
        - network_mode: host
        - cap_add:
            - CAP_NET_ADMIN
            - CAP_NET_BIND_SERVICE
        - devices: /dev/net/tun
        - require:
            - test: container
            - docker_image: mekbuda
            - file: mekbuda
        - environment:
            - HOME=/root
            - HOSTNAME={{ grains['fqdn'] }}
        - watch:
            - file: mekbuda
