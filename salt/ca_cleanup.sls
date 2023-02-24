smallstep:
  docker_image.absent:
    - name: docker.io/smallstep/step-ca
    - require:
      - test: container
  docker_volume.absent:
    - name: smallstep
    - require:
      - test: container
  docker_container.absent:
    - require:
      - test: container
