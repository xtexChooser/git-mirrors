# ========================= Atremis Executables =========================

$(call fs-file)
V_PATH		= /usr/local/bin/atre
V_SYMLINK	= $(ATRE_DIR)/services/atremis/bin/atre
$(call end)

$(call fs-file)
V_PATH		= /usr/local/bin/tiang
V_SYMLINK	= $(ATRE_DIR)/services/atremis/bin/tiang
$(call end)

# ========================= Atremis Systemd Services =========================

$(call fs-file)
V_PATH		= $(SYSTEMD_UNITS_DIR)/atre-pull.service
V_COPY		= $(ATRE_DIR)/services/atremis/systemd/atre-pull.service
V_POST		= systemd-daemon-reload
$(call end)

$(call fs-file)
V_PATH		= $(SYSTEMD_UNITS_DIR)/atre-pull.timer
V_COPY		= $(ATRE_DIR)/services/atremis/systemd/atre-pull.timer
V_DEPS		+= $(SYSTEMD_UNITS_DIR)/atre-pull.service
V_POST		= systemd-daemon-reload
$(call end)

$(call systemd-unit)
V_UNIT		= atre-pull.timer
V_ENABLED	= y
V_RUNNING	= y
$(call end)

# ========================= dinit =========================
$(call package)
V_PKG		= dinit
V_INSTALLED	= y
V_INST_FILE	= /usr/bin/dinit
$(call end)

$(call package)
V_PKG		= dinit-systemd
V_INSTALLED	= y
V_INST_FILE	= /usr/lib/systemd/system/dinit.service
$(call end)

$(call systemd-unit)
V_UNIT		= dinit.service
V_ENABLED	= y
V_RUNNING	= y
V_DEPS		= pkg-dinit-systemd $(DINITD_DIR)/boot
$(call end)

$(call fs-directory)
V_PATH		= $(DINITD_DIR)
V_EXIST		= y
$(call end)

dinit-configs-dir := $(STATES_DIR)/services/atremis/dinit.d
dinit-configs := $(patsubst $(dinit-configs-dir)/%,%,$(wildcard $(dinit-configs-dir)/*))

define dinit-config-template
$$(call fs-file)
V_PATH		= $(DINITD_DIR)/$1
V_TEMPLATE	= bash-tpl $(dinit-configs-dir)/$1
$$(call end)
endef
$(foreach dinit-config,$(dinit-configs),$(eval $(call dinit-config-template,$(dinit-config))))

$(call fs-directory)
V_PATH		= $(DINITD_DIR)/boot.d
V_EXIST		= y
$(call end)

$(call add-fs-directory,/var/log/atremis)

# ========================= cronie =========================
$(call package)
V_PKG		= cronie
V_INSTALLED	= y
V_INST_FILE	= /usr/bin/crond
$(call end)

$(call systemd-unit)
V_UNIT		= cronie.service
V_ENABLED	= y
V_RUNNING	= y
V_DEPS		= pkg-cronie
$(call end)

# ========================= packages =========================
$(call package)
V_PKG		= podman
V_INSTALLED	= y
V_INST_FILE	= /usr/bin/podman
$(call end)
$(call run-on-apply, pkg-podman)

$(call package)
V_PKG		= jq
V_INSTALLED	= y
V_INST_FILE	= /usr/bin/jq
$(call end)

# ========================= cron jobs =========================

$(call fs-file)
V_PATH		= /etc/cron.d/update-containers
V_TEMPLATE	= bash-tpl $(STATES_DIR)/services/atremis/cron-update-containers
V_DEP_VARS	+= STATES_DIR
V_POST		+= systemd-restart E_UNIT=cronie.service
$(call end)

# ========================= maintainer packages =========================
$(call package)
V_PKG		= ripgrep
V_INSTALLED	= y
V_INST_FILE	= /usr/bin/rg
$(call end)
$(call package)
V_PKG		= neovim
V_INSTALLED	= y
V_INST_FILE	= /usr/bin/nvim
$(call end)
$(call package)
V_PKG		= fish
V_INSTALLED	= y
V_INST_FILE	= /usr/bin/fish
$(call end)
