base:
    '*': 
        - container
        - bird
        - router.ip_fwd
        - router.wireguard
#        - firewalld.base
        - caddy
        - node_tls.agent
    'spica:signer:true':
        - match: pillar
        - spica.signer
#    'service:etcd:true':
#        - match: pillar
#        - etcd.base
    'service:ntfy:true':
        - match: pillar
        - ntfy
    'watchtower:enable:true':
        - match: pillar
#        - watchtower
    'nl-alk1':
        - mekbuda
        - openldap.slapd
        - pgsql
        - mediawiki
