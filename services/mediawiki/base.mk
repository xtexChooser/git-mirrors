$(call load-state, services/caddy)

$(call x-container-service)
V_SERVICE	= mediawiki
V_SVCDEPS	+= /etc/mediawiki/LocalSettings.php
V_SVCDEPS	+= /var/run/mediawiki /var/lib/mediawiki
V_ARGS		+= --mount=type=bind,src=/etc/mediawiki,dst=/etc/mediawiki,ro=true
V_ARGS		+= --mount=type=bind,src=/var/run/mediawiki,dst=/var/run/mediawiki
V_ARGS		+= --mount=type=bind,src=/var/lib/mediawiki,dst=/var/lib/mediawiki
V_ARGS		+= --memory=64M
V_ARGS 		+= codeberg.org/xvnet/x-mediawiki-php:latest
$(call end)

$(call fs-file)
V_PATH		= /etc/mediawiki/LocalSettings.php
V_TEMPLATE	= bash-tpl $(STATES_DIR)/services/mediawiki/LocalSettings.php
$(call end)

CADDY_INCLUDES += $(STATES_DIR)/services/mediawiki/Caddyfile

$(call add-fs-directory,/var/run/mediawiki)
$(call add-fs-directory,/var/lib/mediawiki)
