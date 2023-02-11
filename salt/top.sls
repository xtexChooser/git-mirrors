base:
    '*': 
        - container
        - router.bird
        - router.ip_fwd
        - router.wireguard
    'nl-alk1':
        - salt-master-cd
