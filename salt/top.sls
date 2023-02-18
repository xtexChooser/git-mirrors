base:
    '*': 
        - container
        - router.bird
        - router.ip_fwd
        - router.wireguard
        - firewalld.base
    'I@service.salt-master:true':
        - salt-master-cd
