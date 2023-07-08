SYSTEMD_UNIT_VARS=V_TARGET_NAME V_UNIT V_ENABLED V_DISABLED V_RUNNING V_STOPPED V_USER V_SYSTEMCTL V_POST V_DEPS
SYSTEMCTL ?= systemctl
SYSTEMCTL_USER ?= $(SYSTEMCTL) --user

define systemd-unit0
$(eval V_TARGET_NAME?=systemd-$(V_UNIT))
$(eval V_SYSTEMCTL ?= $(if $(V_USER),$(SYSTEMCTL_USER),$(SYSTEMCTL)))
$(if $(V_ENABLED),$(if $(V_DISABLED),$(error Both V_ENABLED and V_DISABLED is defined for $(V_UNIT))))
$(if $(V_RUNNING),$(if $(V_STOPPED),$(error Both V_RUNNING and V_STOPPED is defined for $(V_UNIT))))

$(call mktrace,Define systemd unit target: $(V_UNIT))
$(call mktrace-vars,$(SYSTEMD_UNIT_VARS))
$(call apply-target,$(V_TARGET_NAME))
$(call vt-target,$(V_TARGET_NAME))
$(V_TARGET_NAME): $(V_DEPS)
	export E_UNIT=$(V_UNIT)
$(if $(V_ENABLED),
	if ! $(V_SYSTEMCTL) is-enabled $(V_UNIT) > /dev/null; then
		$(call log, Enable SD unit $(V_UNIT))
		$(V_SYSTEMCTL) enable $(V_UNIT)
		$(call succ, Enabled SD unit $(V_UNIT))
		$(if $(V_POST),E_EVENT=systemd.enabled $(MAKE) $(MAKE_FLAGS) $(V_POST))
	fi
)
$(if $(V_DISABLED),
	if $(V_SYSTEMCTL) is-enabled $(V_UNIT) > /dev/null; then
		$(call log, Disable SD unit $(V_UNIT))
		$(V_SYSTEMCTL) enable $(V_UNIT)
		$(call succ, Disabled SD unit $(V_UNIT))
		$(if $(V_POST),E_EVENT=systemd.disabled $(MAKE) $(MAKE_FLAGS) $(V_POST))
	fi
)
$(if $(V_RUNNING),
	if ! $(V_SYSTEMCTL) is-active $(V_UNIT) > /dev/null; then
		$(call log, Activate SD unit $(V_UNIT))
		$(V_SYSTEMCTL) start $(V_UNIT)
		$(call succ, Started SD unit $(V_UNIT))
		$(if $(V_POST),E_EVENT=systemd.activated $(MAKE) $(MAKE_FLAGS) $(V_POST))
	fi
)
$(if $(V_STOPPED),
	if $(V_SYSTEMCTL) is-active $(V_UNIT) > /dev/null; then
		$(call log, Deactivate SD unit $(V_UNIT))
		$(V_SYSTEMCTL) stop $(V_UNIT)
		$(call succ, Stopped SD unit $(V_UNIT))
		$(if $(V_POST),E_EVENT=systemd.deactivated $(MAKE) $(MAKE_FLAGS) $(V_POST))
	fi
)

$(call unset-vars)
endef

$(call define-func, systemd-unit)
