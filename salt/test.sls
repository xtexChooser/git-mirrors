echo:
    docker_image.pulled:
        - name: k8s.gcr.io/echoserver
        - tag: 1.4
        - force: True
    docker_container.running:
        - image: k8s.gcr.io/echoserver
        - require:
            - docker_image.pulled: echo
        - port_bindings:
            - 8081:8080
