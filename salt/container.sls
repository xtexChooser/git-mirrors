podman:
    pkg.installed:
        - pkgs:
            - podman
            - podman-docker
    service.running:
        - enable: True
        - require:
            - pkg: podman

container:
    test.nop:
        - use:
            - pkg: podman
            - service: podman
