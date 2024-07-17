SYSTEMCTL = systemctl
SYSTEMCTL_USER = $(SYSTEMCTL) --user
SYSTEMD_UNITS_DIR ?= /etc/systemd/system
SYSTEMD_USER_UNITS_DIR ?= $(HOME)/.config/systemd/user

SYSTEMD_UNIT_VARS = V_TARGET_NAME V_UNIT V_ENABLED V_RUNNING V_USER V_SYSTEMCTL V_POST $(v-deps-var)
define systemd-unit0
$(eval V_TARGET_NAME?=systemd-$(V_UNIT))
$(eval V_SYSTEMCTL ?= $(if $(V_USER),$(SYSTEMCTL_USER),$(SYSTEMCTL)))
$(if $(findstring .,$(V_UNIT)),,$(call mkerr, V_UNIT $(V_UNIT) should include the suffix, e.g. .service and .timer))

$(call mktrace, Define systemd unit target: $(V_UNIT))
$(call mktrace-vars,$(SYSTEMD_UNIT_VARS))
$(call apply-target,$(V_TARGET_NAME))
$(call vt-target,$(V_TARGET_NAME))
$(V_TARGET_NAME): $(v-deps) $(if $(V_USER),$(call file-imp-dep,$(SYSTEMD_USER_UNITS_DIR)/$(V_UNIT)), \
		$(call file-imp-dep,$(SYSTEMD_UNITS_DIR)/$(V_UNIT))) \
		| $(v-deps-order)
	export E_MAJOR=systemd E_UNIT=$(V_UNIT)
$(if $(call is-true,$(V_ENABLED)),
	if ! $(V_SYSTEMCTL) is-enabled $(V_UNIT) $(DROP_STDOUT); then
		$(V_SYSTEMCTL) enable $(V_UNIT)
		$(call succ, Enabled SD unit $(V_UNIT))
		$(call vpost, E_MINOR=enabled)
	fi
)
$(if $(call is-false,$(V_ENABLED)),
	if $(V_SYSTEMCTL) is-enabled $(V_UNIT) $(DROP_STDOUT); then
		$(V_SYSTEMCTL) enable $(V_UNIT)
		$(call succ, Disabled SD unit $(V_UNIT))
		$(call vpost, E_MINOR=disabled)
	fi
)
$(if $(call is-true,$(V_RUNNING)),
	if ! $(V_SYSTEMCTL) is-active $(V_UNIT) $(DROP_STDOUT); then
		$(V_SYSTEMCTL) start $(V_UNIT)
		$(call succ, Started SD unit $(V_UNIT))
		$(call vpost, E_MINOR=activated)
	fi
)
$(if $(call is-false,$(V_RUNNING)),
	if $(V_SYSTEMCTL) is-active $(V_UNIT) $(DROP_STDOUT); then
		$(V_SYSTEMCTL) stop $(V_UNIT)
		$(call succ, Stopped SD unit $(V_UNIT))
		$(call vpost, E_MINOR=deactivated)
	fi
)

$(call unset-vars)
endef

$(call define-func, systemd-unit)

$(call vt-target, systemd-restart systemd-reload systemd-daemon-reload systemd-sudo-daemon-reload systemd-user-daemon-reload)
systemd-restart:
	$(SYSTEMCTL) restart $(E_UNIT)
	$(call succ, Restarted SD unit $(E_UNIT))
systemd-reload:
	$(SYSTEMCTL) reload $(E_UNIT)
	$(call succ, Reloaded SD unit $(E_UNIT))
systemd-daemon-reload:
	$(SYSTEMCTL) daemon-reload
	$(call succ, Reloaded systemd daemon)
systemd-sudo-daemon-reload:
	sudo $(SYSTEMCTL) daemon-reload
	$(call succ, Reloaded systemd daemon)
systemd-user-daemon-reload:
	$(SYSTEMCTL_USER) daemon-reload
	$(call succ, Reloaded user systemd daemon)
