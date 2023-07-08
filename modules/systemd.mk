SYSTEMD_UNIT_VARS=V_TARGET_NAME V_UNIT V_ENABLED V_DISABLED V_RUNNING V_STOPPED V_USER
define systemd-unit0
$(eval V_TARGET_NAME?=systemd-$(V_UNIT))
$(call mktrace,Define systemd unit target: $(V_UNIT))
$(call mktrace-vars,$(SYSTEMD_UNIT_VARS))

$(call apply-target,$(V_TARGET_NAME))
$(call vt-target,$(V_TARGET_NAME))
$(V_TARGET_NAME):
	$(call log,test)

$(call unset-vars,$(SYSTEMD_UNIT_VARS))
endef

$(call define-func, systemd-unit)
