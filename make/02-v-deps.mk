v-deps-var := V_DEPS V_DEP_VARS

define v-deps
$(V_DEPS) $(call dep-vars,$(V_DEP_VARS))
endef

define v-var-dep-files
$(foreach var,$(V_DEP_VARS),$(saved-var-$(var)))
endef
