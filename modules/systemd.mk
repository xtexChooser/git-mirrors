SYSTEMCTL = systemctl
SYSTEMCTL_USER = $(SYSTEMCTL) --user

SYSTEMD_UNIT_VARS=V_TARGET_NAME V_UNIT V_ENABLED V_DISABLED V_RUNNING V_STOPPED V_USER V_SYSTEMCTL V_POST V_DEPS
define systemd-unit0
$(eval V_TARGET_NAME?=systemd-$(V_UNIT))
$(eval V_SYSTEMCTL ?= $(if $(V_USER),$(SYSTEMCTL_USER),$(SYSTEMCTL)))
$(if $(V_ENABLED),$(if $(V_DISABLED),$(error Both V_ENABLED and V_DISABLED is defined for $(V_UNIT))))
$(if $(V_RUNNING),$(if $(V_STOPPED),$(error Both V_RUNNING and V_STOPPED is defined for $(V_UNIT))))
$(if $(findstring .,$(V_UNIT)),,$(error V_UNIT $(V_UNIT) should include the suffix, e.g. .service and .timer))

$(call mktrace,Define systemd unit target: $(V_UNIT))
$(call mktrace-vars,$(SYSTEMD_UNIT_VARS))
$(call apply-target,$(V_TARGET_NAME))
$(call vt-target,$(V_TARGET_NAME))
$(V_TARGET_NAME): $(V_DEPS)
	export E_MAJOR=systemd E_UNIT=$(V_UNIT)
$(if $(V_ENABLED),
	if ! $(V_SYSTEMCTL) is-enabled $(V_UNIT) > /dev/null; then
		$(V_SYSTEMCTL) enable $(V_UNIT)
		$(call succ, Enabled SD unit $(V_UNIT))
		$(if $(V_POST),E_MINOR=enabled $(MAKE) $(MAKE_FLAGS) $(V_POST))
	fi
)
$(if $(V_DISABLED),
	if $(V_SYSTEMCTL) is-enabled $(V_UNIT) > /dev/null; then
		$(V_SYSTEMCTL) enable $(V_UNIT)
		$(call succ, Disabled SD unit $(V_UNIT))
		$(if $(V_POST),E_MINOR=disabled $(MAKE) $(MAKE_FLAGS) $(V_POST))
	fi
)
$(if $(V_RUNNING),
	if ! $(V_SYSTEMCTL) is-active $(V_UNIT) > /dev/null; then
		$(V_SYSTEMCTL) start $(V_UNIT)
		$(call succ, Started SD unit $(V_UNIT))
		$(if $(V_POST),E_MINOR=activated $(MAKE) $(MAKE_FLAGS) $(V_POST))
	fi
)
$(if $(V_STOPPED),
	if $(V_SYSTEMCTL) is-active $(V_UNIT) > /dev/null; then
		$(V_SYSTEMCTL) stop $(V_UNIT)
		$(call succ, Stopped SD unit $(V_UNIT))
		$(if $(V_POST),E_MINOR=deactivated $(MAKE) $(MAKE_FLAGS) $(V_POST))
	fi
)

$(call unset-vars)
endef

$(call define-func, systemd-unit)

$(call vt-target,systemd-restart systemd-reload systemd-daemon-reload systemd-user-daemon-reload)
systemd-restart:
	$(SYSTEMCTL) restart $(E_UNIT)
	$(call succ, Restarted SD unit $(E_UNIT))
systemd-reload:
	$(SYSTEMCTL) reload $(E_UNIT)
	$(call succ, Reloaded SD unit $(E_UNIT))
systemd-daemon-reload:
	$(SYSTEMCTL) daemon-reload
	$(call succ, Reloaded systemd daemon)
systemd-user-daemon-reload:
	$(SYSTEMCTL_USER) daemon-reload
	$(call succ, Reloaded user systemd daemon)
