### tiang::target koit ssh://koit.s.xvnet0.eu.org

XVNET_NUM := 2

### tiang::tag koit bird
$(call load-state, services/bird)

### tiang::tag koit caddy
$(call load-state, services/caddy)
