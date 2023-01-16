echo:
    docker_image.present:
        - name: k8s.gcr.io/echoserver
        - tag: 1.4
        - force: True
    docker_container.running:
        - image: k8s.gcr.io/echoserver
        - require:
            - docker_image: echo
        - port_bindings:
            - 8081:8080
