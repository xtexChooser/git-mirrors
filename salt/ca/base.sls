smallstep:
  docker_image.present:
    - name: docker.io/smallstep/step-ca
    - tag: {{ pillar['version']['smallstep'] }}
    - require:
      - test: container
  docker_volume.present:
    - name: smallstep
    - driver: local
    - require:
      - test: container
  docker_container.running:
    - image: docker.io/smallstep/step-ca:{{ pillar['version']['smallstep'] }}
    - binds:
      - smallstep:/home/step:rw
    - port_bindings:
      - 9000:9465
    - require:
      - test: container
      - docker_image: smallstep
      - docker_volume: smallstep
    - environment:
      - HOME=/root
      - HOSTNAME={{ grains['fqdn'] }}
      - DOCKER_STEPCA_INIT_NAME=XTEX-VNET Trust
      - DOCKER_STEPCA_INIT_PROVISIONER_NAME=root
      - DOCKER_STEPCA_INIT_ADMIN_SUBJECT=xtex
      - DOCKER_STEPCA_INIT_SSH=true
      - DOCKER_STEPCA_INIT_ACME=true
      - DOCKER_STEPCA_INIT_REMOTE_MANAGEMENT=true
      - DOCKER_STEPCA_INIT_DNS_NAMES=localhost,nl-alk1.svr.xvnet.eu.org,ca.xvnet.eu.org
    - memory: 64M
