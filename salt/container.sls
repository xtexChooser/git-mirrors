container:
    pkg.installed:
        - pkgs:
            - podman
    test.nop:
        - use:
            - pkgs: container
