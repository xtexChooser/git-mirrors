STAMP_VARS = V_TARGET_NAME V_NAME V_POST $(v-deps-var) V_PATH
define stamp0
$(eval V_TARGET_NAME?=stamp-$(V_NAME))
$(eval V_PATH?=$(STAMPS_DIR)/stamp-$(V_NAME))

$(call mktrace, Define stamp target: $(V_UNIT))
$(call mktrace-vars,$(STAMP_VARS))

$(call apply-target,$(V_TARGET_NAME))
$(call vt-target,$(V_TARGET_NAME))
$(V_TARGET_NAME): $(V_PATH)

$(V_PATH): $(v-deps) | $(v-deps-order)
	export E_MAJOR=stamp E_NAME=$(V_NAME) E_PATH=$(V_PATH)
	$$(TOUCH) $$@
	$(call vpost, E_MINOR=refreshed)
	$(call succ, Updated stamp $(V_NAME))

$(call unset-vars)
endef

$(call define-func, stamp)
