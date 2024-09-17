$(call load-state, services/caddy)

$(call x-container-service)
V_SERVICE	= populus
V_DEPS		+= /etc/populus/populus.cfg
V_DEPS_ORD	+= /var/run/populus
V_ARGS		+= --mount=type=bind,src=/etc/populus,dst=/etc/populus,ro=true
V_ARGS		+= --mount=type=bind,src=/srv/secrets/populus,dst=/srv/secrets/populus,ro=true
V_ARGS		+= --mount=type=bind,src=/var/run/populus,dst=/var/run/populus,chown=true
V_ARGS		+= --memory=256M
V_ARGS		+= --hostname=idp.$(HOSTNAME)
V_ARGS 		+= codeberg.org/xens/populus:latest
$(call end)

$(call fs-file)
V_PATH		= /etc/populus/populus.cfg
V_TEMPLATE	= bash-tpl $(STATES_DIR)/services/populus/populus.cfg
$(call end)

$(call add-fs-directory,/var/run/populus)
