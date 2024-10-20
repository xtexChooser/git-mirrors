$(call x-container-service)
V_SERVICE	= caddy
V_DEPS_ORD	+= /var/lib/caddy /etc/caddy/Caddyfile
V_ARGS		+= --cap-add=CAP_NET_BIND_SERVICE
V_ARGS		+= --env HOME=/root
V_ARGS		+= --mount=type=bind,src=/etc/caddy,dst=/etc/caddy,ro=true
V_ARGS		+= --mount=type=bind,src=/var/run,dst=/var/run
V_ARGS		+= --mount=type=bind,src=/var/lib/caddy,dst=/data/caddy
V_ARGS		+= --publish=80:80/tcp --publish=80:80/udp
V_ARGS		+= --publish=443:443/tcp --publish=443:443/udp
V_ARGS		+= --memory=64M
$(call invoke-hooks,caddy-container-opts)
V_ARGS 		+= codeberg.org/xens/x-caddy
$(call end)

$(call add-fs-directory,/var/lib/caddy)

CADDY_INCLUDES :=
$(call invoke-hooks,caddy-configs)
$(call fs-file)
V_PATH		= /etc/caddy/Caddyfile
V_TEMPLATE	= bash-tpl $(STATES_DIR)/services/caddy/Caddyfile
V_DEPS		+= $(wildcard $(STATES_DIR)/services/caddy/config/*.caddyfile) $(CADDY_INCLUDES)
$(call end)

$(call cmd-stamp)
V_NAME		= caddy-reload
V_CMD		= $(STATES_DIR)/atre svc caddy reload
V_DEPS		+= /etc/caddy/Caddyfile
V_DEPS_ORD	+= dinit-caddy
$(call end)
