### tiang::target nl-alk1 ssh://nl-alk1.svr.xvnet.eu.org

BIRD_ROUTER_ID := 5.255.109.94
XVNET_NUM := 1

### tiang::tag nl-alk1 bird
$(call load-state, services/bird)

### tiang::tag nl-alk1 caddy
$(call load-state, services/caddy)

### tiang::tag nl-alk1 bird
$(call load-state, services/ntfy)
