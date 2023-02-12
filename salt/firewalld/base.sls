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

firewalld-public:
    firewalld.present:
        - default: True
        - prune_services: False
        - services:
            - dhcpv6-client
            - ssh
            - cockpit
            - http
            - https
{% if pillar['salt-master'] is defined -%}
            - salt-master
{% endif -%}
# dhcpv6-client ssh cockpit http https salt-master syncthing syncthing-gui
# ports: 10300-10400/udp 6443/tcp 10250/tcp 6443/udp 8448/tcp
