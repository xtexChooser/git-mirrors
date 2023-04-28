include:
    - container

pgsql:
    docker_container.running:
        - image: docker.io/library/postgres:alpine
        - binds:
            - /var/lib/pgsql:/var/lib/postgresql
            - /opt:/opt:ro
        - require:
            - test: container
        - memory: 256M
        - hostname: pgsql
        - environment:
            - HOME: /root
            - POSTGRES_PASSWORD: "{{ pillar['pgsql']['su_passwd'] }}"
            - POSTGRES_INITDB_ARGS: --data-checksums
        - port_bindings:
            - 5432:5432
