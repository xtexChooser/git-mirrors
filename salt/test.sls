echo:
    docker_image.absent:
        - images:
            - k8s.gcr.io/echoserver
        - force: True
    docker_container.absent:
        - force: True
