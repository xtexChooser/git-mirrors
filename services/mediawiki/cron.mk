$(call fs-file)
V_PATH		= /etc/cron.d/mediawiki
V_TEMPLATE	= bash-tpl $(STATES_DIR)/services/mediawiki/crontab
V_DEP_VARS	+= STATES_DIR
V_POST		+= systemd-restart E_UNIT=cronie.service
V_DEPS		= pkg-cronie
$(call end)
