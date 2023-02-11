podman:
    pkg.installed:
        - pkgs:
            - podman
            - podman-docker
    service.running:
        - enable: True

container:
    test.nop:
        - use:
            - pkg: podman
