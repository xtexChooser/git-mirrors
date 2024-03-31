### tiang::target opilio ssh://opilio.s.xvnet0.eu.org

BIRD_ROUTER_ID := 5.255.109.94
XVNET_NUM := 1

### tiang::tag opilio bird
$(call load-state, services/bird)

### tiang::tag opilio caddy
$(call load-state, services/caddy)

### tiang::tag opilio ntfy
$(call load-state, services/ntfy)

### tiang::tag opilio bind
$(call load-state, services/bind)

### tiang::tag opilio postgres
### tiang::tag opilio balaro
$(call load-state, services/postgres/balaro)
