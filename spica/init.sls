spica:
    docker_image.present:
        - name: codeberg.org/xvnet/spica
        - tag: latest
        - require:
            - test: container
