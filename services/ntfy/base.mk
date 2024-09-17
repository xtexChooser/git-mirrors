$(call load-state, services/caddy)

$(call x-container-service)
V_SERVICE	= ntfy
V_DEPS		+= /etc/ntfy/server.yml
V_DEPS_ORD	+= /var/run/ntfy /var/lib/ntfy /var/cache/ntfy /var/log/ntfy
V_ARGS		+= --mount=type=bind,src=/etc/ntfy,dst=/etc/ntfy,ro=true
V_ARGS		+= --mount=type=bind,src=/var/run/ntfy,dst=/var/run/ntfy
V_ARGS		+= --mount=type=bind,src=/var/lib/ntfy,dst=/var/lib/ntfy
V_ARGS		+= --mount=type=bind,src=/var/cache/ntfy,dst=/var/cache/ntfy
V_ARGS		+= --mount=type=bind,src=/var/log/ntfy,dst=/var/log/ntfy
V_ARGS		+= --memory=64M
V_ARGS 		+= codeberg.org/xens/ntfy:latest
$(call end)

$(call fs-file)
V_PATH		= /etc/ntfy/server.yml
V_TEMPLATE	= bash-tpl $(STATES_DIR)/services/ntfy/server.yml
$(call end)

$(call add-fs-directory,/var/run/ntfy)
$(call add-fs-directory,/var/lib/ntfy)
$(call add-fs-directory,/var/cache/ntfy)
$(call add-fs-directory,/var/log/ntfy)
