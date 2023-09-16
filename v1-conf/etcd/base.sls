etcd:
  docker_image.present:
    - name: gcr.io/etcd-development/etcd
    - tag: {{ pillar['version']['etcd'] }}
    - require:
      - test: container
  docker_volume.present:
    - name: etcd
    - driver: local
    - require:
      - test: container
  docker_container.running:
    - image: gcr.io/etcd-development/etcd:{{ pillar['version']['etcd'] }}
    - binds:
      - etcd:/var/lib/etcd-data:rw
    - port_bindings:
      - 2379
      - 2380
    - require:
      - test: container
      - docker_image: etcd
      - docker_volume: etcd
      - service: etcd
    - environment:
      - HOME=/root
      - HOSTNAME={{ grains['fqdn'] }}
    - memory: 64M
    - cmd: etcd --data-dir=/var/lib/etcd-data --name {{ pillar['name'] }} \
        --discovery-srv etcd.infra.xvnet.eu.org \
        --initial-advertise-peer-urls https://{{ grains['fqdn'] }}:2380 \
        --initial-cluster-token xvnet-main-etcd-cluster \
        --initial-cluster-state new \
        --advertise-client-urls https://{{ grains['fqdn'] }}:2379 \
        --listen-client-urls https://0.0.0.0:2379 \
        --listen-peer-urls https://0.0.0.0:2380 \
        --client-cert-auth --trusted-ca-file=/path/to/ca-client.crt \
        --cert-file=/path/to/infra0-client.crt --key-file=/path/to/infra0-client.key \
        --peer-client-cert-auth --peer-trusted-ca-file=ca-peer.crt \
        --peer-cert-file=/path/to/infra0-peer.crt --peer-key-file=/path/to/infra0-peer.key \
  pkg.installed:
    - pkgs:
      - etcd-client
      - etcd-server
  service.dead:
    - enable: False
    - require:
      - pkg: etcd
