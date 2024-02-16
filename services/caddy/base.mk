$(call fs-file)
V_PATH		= $(DINITD_DIR)/caddy
V_TEMPLATE	= bash-tpl $(STATES_DIR)/services/caddy/dinit-caddy
$(call end)

$(call fs-file)
V_PATH		= $(DINITD_DIR)/boot.d/caddy
V_SYMLINK	= ../caddy
$(call end)

$(call dinit-service)
V_SERVICE	= caddy
V_RUNNING	= y
$(call end)
