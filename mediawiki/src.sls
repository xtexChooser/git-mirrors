include:
    - container

codeberg.org/xvnet/mediawiki-src:
    docker_image.present:
        - tag: latest
        - require:
            - test: container

mediawiki-src:
    docker_volume.present:
        - driver: image
        - driver_opts:
            - image: codeberg.org/xvnet/mediawiki-src:latest
        - require:
            - docker_image: codeberg.org/xvnet/mediawiki-src
