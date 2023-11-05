$(call fs-file)
V_PATH		= /usr/local/bin/atre
V_SYMLINK	= $(ATRE_DIR)/services/atremis/bin/atre
$(call end)

$(call fs-file)
V_PATH		= /usr/local/bin/tiang
V_SYMLINK	= $(ATRE_DIR)/services/atremis/bin/tiang
$(call end)

$(call fs-file)
V_PATH		= /etc/systemd/system/atre-pull.service
V_COPY		= $(ATRE_DIR)/services/atremis/systemd/atre-pull.service
V_POST		= systemd-sudo-daemon-reload
$(call end)

$(call fs-file)
V_PATH		= /etc/systemd/system/atre-pull.timer
V_COPY		= $(ATRE_DIR)/services/atremis/systemd/atre-pull.timer
V_DEPS		+= /etc/systemd/system/atre-pull.service
V_POST		= systemd-sudo-daemon-reload
$(call end)

$(call systemd-unit)
V_UNIT		= atre-pull.timer
V_ENABLED	= y
V_DEPS		= /etc/systemd/system/atre-pull.timer
$(call end)
