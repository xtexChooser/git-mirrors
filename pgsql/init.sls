include:
    - container

pgsql:
    docker_container.running:
        - image: docker.io/library/postgres:alpine
        - binds:
            - /var/lib/pgsql/data:/var/lib/postgresql/data
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
        - command: -c ssl_cert_file=\"/opt/node_tls.crt\"
            -c ssl_key_file=\"/opt/node_tls.key\"
            -c ssl=\"on\"
