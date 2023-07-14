$(call systemd-unit)
V_UNIT		= caddy.service
V_ENABLED	= n
V_RUNNING	= n
V_DEPS		= pkg-caddy
$(call end)

$(call package)
V_PKG		= caddy
V_INSTALLED	= n
V_INST_FILE	= /usr/bin/caddy
$(call end)

$(call cmd)
V_NAME		= sd-reload-caddy
V_CMD		= systemd reload caddy
$(call end)

$(call refresh-timer)
V_NAME		= caddy-daily
#V_POST		= pkg-refresh cmd-sd-reload-caddy
#V_TIME		= 10secs
V_TIME		= 1days
$(call end)

#$(call hostname)
#V_HOSTNAME	= a-caddy-servier
#$(call end)

$(call directory)
V_PATH		= $(BUILD_DIR)/test-caddy
V_EXIST		= y
#V_USER		= root
#V_GROUP		= root
#V_ACCESS	= 0700
$(call end)
