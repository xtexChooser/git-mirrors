echo:
    docker_image.absent:
        - name: k8s.gcr.io/echoserver
        - tag: 1.4
        - force: True
    docker_container.absent:
        - force: True
