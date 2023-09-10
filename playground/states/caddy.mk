$(call systemd-unit)
V_UNIT		= caddy.service
V_ENABLED	= n
V_RUNNING	= n
V_DEPS		= pkg-caddy
$(call end)

$(call package)
V_PKG		= caddy
#V_INSTALLED	= y
#V_INST_FILE	= /usr/bin/caddy
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

$(call fs-directory)
V_PATH		= $(BUILD_DIR)/test-caddy
V_EXIST		= y
#V_USER		= root
#V_GROUP		= root
V_RECURSIVE	= y
V_ACCESS	= 0777
$(call end)

$(call fs-file)
V_PATH		= $(BUILD_DIR)/test-caddy/Caddyfile-old
V_EXIST		= n
$(call end)

$(call fs-file)
V_PATH		= $(BUILD_DIR)/test-caddy/Caddyfile-empty
V_CREATE	= empty
$(call end)

$(call fs-file)
V_PATH		= $(BUILD_DIR)/test-caddy/test.html
V_CREATE	= download URL="https://raw.githubusercontent.com/cbracco/html5-test-page/master/index.html"
$(call end)
