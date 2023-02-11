container:
    pkgs.installed:
        - pkgs:
            - podman
    test.nop:
        - use:
            - pkgs: container
