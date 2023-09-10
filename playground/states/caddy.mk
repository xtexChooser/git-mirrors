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
V_PATH		= $(BUILD_DIR)/test-caddy/file-old
V_EXIST		= n
$(call end)

$(call fs-file)
V_PATH		= $(BUILD_DIR)/test-caddy/file-empty
V_CREATE	= empty
$(call end)

$(call fs-file)
V_PATH		= $(BUILD_DIR)/test-caddy/test.html
V_CREATE	= download URL="https://raw.githubusercontent.com/cbracco/html5-test-page/master/index.html"
$(call end)

$(call fs-file)
V_PATH		= $(BUILD_DIR)/test-caddy/file-template-bash-tpl
V_TEMPLATE	= bash-tpl $(STATES_DIR)/caddy/bash-tpl.txt
$(call end)

$(call fs-file)
V_PATH		= $(BUILD_DIR)/test-caddy/file-template-mo
V_TEMPLATE	= mo $(STATES_DIR)/caddy/mo.txt
$(call end)

$(call fs-file)
V_PATH		= $(BUILD_DIR)/test-caddy/file-template-envsubst
V_TEMPLATE	= envsubst $(STATES_DIR)/caddy/envsubst.txt
$(call end)

$(call fs-file)
V_PATH		= $(BUILD_DIR)/test-caddy/file-template-nop
V_TEMPLATE	= nop $(STATES_DIR)/caddy/mo.txt
$(call end)

$(call fs-file)
V_PATH		= $(BUILD_DIR)/test-caddy/file-copy
V_COPY		= $(STATES_DIR)/caddy/mo.txt
$(call end)

#$(call fs-file)
#V_PATH		= $(BUILD_DIR)/test-caddy/file-exist
#V_ACCESS	= 0777
#$(call end)

$(call fs-line)
V_NAME = caddy-line
V_PATH = $(BUILD_DIR)/test-caddy/file-line.txt
V_FLAGS =append
V_MATCH = Value\s*=\s*False
V_LINE = Value = True
$(call end)

$(call fs-file)
V_PATH		= $(BUILD_DIR)/test-caddy/Caddyfile
V_COPY		= $(STATES_DIR)/caddy/Caddyfile
$(call end)

CADDY_DIR=$(shell readlink -e $(BUILD_DIR)/test-caddy)
$(call fs-file)
V_PATH		= $(BUILD_DIR)/test-caddy/caddy.yaml
V_TEMPLATE	= bash-tpl $(STATES_DIR)/caddy/caddy.yaml
V_DEP_VARS	+= CADDY_DIR
$(call end)

$(call stamp)
V_NAME		= caddy-kube-conf-test
V_DEPS		= $(BUILD_DIR)/test-caddy/caddy.yaml
$(call end)

$(call podman-kube)
V_NAME		= caddy
V_POD		= caddy
V_FILE		= $(BUILD_DIR)/test-caddy/caddy.yaml
V_DEPS		+= $(BUILD_DIR)/test-caddy/Caddyfile
$(call end)
