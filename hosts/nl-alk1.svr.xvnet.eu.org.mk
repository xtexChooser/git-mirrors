BIRD_ROUTER_ID := 5.255.109.94
XVNET_NUM := 1
DN42_LOCAL_IP := 172.20.206.$(shell echo "$$((64 + $(XVNET_NUM)))")
XVNET_ASN := $(shell echo "$$((4244310000 + $(XVNET_NUM)))")
XVNET_LOCAL_IP := fd00:443a:ef14:1::$(shell printf '%x:%x' \
	$$(($(XVNET_NUM) / 0x10000)) $$(($(XVNET_NUM) % 0x10000)))

$(call load-state, services/bird)
$(call load-state, services/caddy)
$(call load-state, services/ntfy)
