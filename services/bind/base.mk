$(call x-container-service)
V_SERVICE	= bind
V_DEPS		+= /etc/bind/named.conf
V_SVCDEPS	+= /var/run/bind
V_ARGS		+= --mount=type=bind,src=/etc/bind,dst=/etc/bind,ro=true
V_ARGS		+= --mount=type=bind,src=/var/run/bind,dst=/var/run/bind
V_ARGS		+= --publish=53:53/udp # Do53
V_ARGS		+= --publish=853:853/tcp # DoT
V_ARGS		+= --publish=453:453/tcp # DoH
V_ARGS 		+= codeberg.org/xvnet/bind:latest
$(call end)

$(call add-fs-directory,/var/run/bind)

$(call fs-file)
V_PATH		= /etc/bind/named.conf
V_TEMPLATE	= bash-tpl $(STATES_DIR)/services/bind/conf/named.conf
V_DEPS		+= $(wildcard $(STATES_DIR)/services/bind/conf/*.conf)
$(call end)
