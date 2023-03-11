spica:
    docker_image.present:
        - name: codeberg.org/xtex-vnet/spica
        - tag: latest
        - force: True
        - require:
            - test: container
