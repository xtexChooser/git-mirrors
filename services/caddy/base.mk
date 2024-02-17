$(call x-container-service)
V_SERVICE	= caddy
V_ARGS		+= --cap-add=CAP_NET_BIND_SERVICE
V_ARGS		+= --env HOME=/root
V_ARGS		+= --hostname=caddy
V_ARGS		+= --mount=type=bind,src=/etc/caddy,dst=/etc/caddy,ro=true
V_ARGS		+= --mount=type=bind,src=/var/run,dst=/var/run
V_ARGS		+= --mount=type=bind,src=/var/lib/caddy,dst=/root/.local/share/caddy
V_ARGS		+= --publish=80:80
V_ARGS		+= --publish=443:443
V_ARGS 		+= codeberg.org/xvnet/x-caddy
$(call end)
