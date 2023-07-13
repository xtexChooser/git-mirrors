REFRESH_TIMER_VARS=V_TARGET_NAME V_NAME V_TIME V_POST V_STAMP_FILE
define refresh-timer0
$(eval V_TARGET_NAME?=refresh-timer-$(V_NAME))
$(eval V_STAMP_FILE?=$(STAMPS_DIR)/target-$(V_TARGET_NAME))

$(call mktrace,Define refresh timer target: $(V_NAME))
$(call mktrace-vars,$(REFRESH_TIMER_VARS))
$(call apply-target,$(V_TARGET_NAME))
$(call vt-target,$(V_TARGET_NAME))

$(V_TARGET_NAME): $(V_STAMP_FILE)
$(V_STAMP_FILE): $(STAMP_REF)
	$$(TOUCH) -r $(STAMP_REF) -d $(V_TIME) $$@
	$$(call succ, Updated refresh timer $$@)
	$(call vpost, E_MAJOR=refresh-timer E_MINOR=run)

$(call unset-vars)
endef

$(call define-func,refresh-timer)
