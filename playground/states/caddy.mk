$(call systemd-unit)
V_UNIT		= caddy.service
V_DISABLE	= y
V_STOPPED	= y
V_DEPS		= pkg-caddy
$(call end)

$(call package)
V_PKG		= caddy
V_REMOVED	= y
V_INST_FILE	= /usr/bin/caddy
$(call end)
