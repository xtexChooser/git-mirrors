$(call systemd-unit)
V_UNIT		= caddy.service
V_ENABLED	= y
V_RUNNING	= y
V_DEPS		= pkg-caddy
$(call end)

$(call package)
V_PKG		= caddy
V_INSTALLED	= y
V_INST_FILE	= /usr/bin/caddy
$(call end)
