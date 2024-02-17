BIRD_ROUTER_ID := 5.255.109.94
DN42_LOCAL_IP := 172.20.206.65
XVNET_ASN := 4244310001
XVNET_LOCAL_IP := fd00:443a:ef14:1::1

$(call load-state, services/bird)
$(call load-state, services/caddy)
$(call load-state, services/ntfy)
