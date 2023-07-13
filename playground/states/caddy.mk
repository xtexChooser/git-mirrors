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

$(call cmd)
V_NAME		= sd-reload-caddy
V_CMD		= systemd reload caddy
$(call end)

$(call refresh-timer)
V_NAME		= caddy-daily
V_POST		= pkg-refresh cmd-sd-reload-caddy
#V_TIME		= 10secs
V_TIME		= 1days
$(call end)

#$(call hostname)
#V_HOSTNAME	= a-caddy-servier
#$(call end)
