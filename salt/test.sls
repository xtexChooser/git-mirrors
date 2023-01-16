echo:
    docker_image.present:
        - name: k8s.gcr.io/echoserver
        - tag: latest
        - force: True
    docker_container.running:
        - image: k8s.gcr.io/echoserver:latest
        - require:
            - docker_image: echo
        - port_bindings:
            - 8081:8080
