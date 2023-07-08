define systemd-unit0
$(eval V_TARGET_NAME?=systemd-$(V_UNIT))
$(call apply-target,$(V_TARGET_NAME))
$(call vt-target,$(V_TARGET_NAME))
$(V_TARGET_NAME):
	echo enablding $(V_TARGET_NAME)
$(call unset-vars, V_TARGET_NAME, V_UNIT, V_ENABLED, V_DISABLED)
endef

$(call define-func, systemd-unit)
