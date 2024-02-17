# $(call fs-file)
# V_PATH		= $(DINITD_DIR)/caddy
# V_TEMPLATE	= bash-tpl $(STATES_DIR)/services/caddy/dinit-caddy
# $(call end)

$(call x-container-service)
V_SERVICE	= caddy
V_ARGS		+= --cap-add=CAP_NET_BIND_SERVICE
V_ARGS		+= --hostname=caddy
V_ARGS		+= --env HOME=/root
V_ARGS		+= --mount=type=bind,src=/etc/caddy,dst=/etc/caddy,ro=true
V_ARGS 		+= codeberg.org/xvnet/x-caddy
$(call end)

# $(call fs-file)
# V_PATH		= $(DINITD_DIR)/boot.d/caddy
# V_SYMLINK	= ../caddy
# $(call end)

# $(call vt-target, caddy-vcmd-start)
# caddy-vcmd-start:
# 	mkdir -p /var/run/caddy
# 	podman run --name caddy \
# 		--rm -d --pidfile=/var/run/containers/caddy.pid \

# $(call cmd-target, caddy-vcmd-stop)
