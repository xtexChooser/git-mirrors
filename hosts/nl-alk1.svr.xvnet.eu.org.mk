BIRD_ROUTER_ID := 5.255.109.94
XVNET_NUM := 1

$(call load-state, services/bird)
$(call load-state, services/caddy)
$(call load-state, services/ntfy)
