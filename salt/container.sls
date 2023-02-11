container:
    pkg.installed:
        - pkgs:
            - podman
    test.nop:
        - use:
            - pkg: container
