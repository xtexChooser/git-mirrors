firewalld:
    pkg.installed:
        - pkgs:
            - firewalld
    service.dead: 
        - require:
            - pkg: firewalld
    test.nop:
        - use:
            - pkg: firewalld
            - service: firewalld
