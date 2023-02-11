base:
    '*': 
        - container
        - router.bird
        - router.ip_fwd
        - router.wireguard
        - firewalld.base
    'nl-alk1':
        - salt-master-cd
