$(call load-state, services/caddy)

# $(call x-container-service)
# V_SERVICE	= ntfy
# V_DEPS		+= /etc/ntfy/server.yml
# V_ARGS		+= --cap-add=CAP_NET_BIND_SERVICE
# V_ARGS		+= --env HOME=/root
# V_ARGS		+= --hostname=ntfy
# V_ARGS		+= --mount=type=bind,src=/etc/ntfy,dst=/etc/ntfy,ro=true
# V_ARGS		+= --mount=type=bind,src=/var/run,dst=/var/run
# V_ARGS		+= --mount=type=bind,src=/var/lib/ntfy,dst=/root/.local/share/ntfy
# V_ARGS		+= --publish=80:80
# V_ARGS		+= --publish=443:443
# V_ARGS 		+= codeberg.org/xvnet/x-ntfy
# $(call end)

$(call fs-file)
V_PATH		= /etc/ntfy/server.yml
V_TEMPLATE	= bash-tpl $(STATES_DIR)/services/ntfy/server.yml
$(call end)

CADDY_INCLUDES += $(STATES_DIR)/services/ntfy/Caddyfile
