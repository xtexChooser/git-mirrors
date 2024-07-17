v-deps-var := V_DEPS V_DEP_VARS V_DEPS_ORD V_DEP_ORDV

define v-deps
$(V_DEPS) $(call dep-vars,$(V_DEP_VARS))
endef

define v-deps-order
$(V_DEPS_ORD) $(call dep-vars,$(V_DEP_ORDV))
endef

define v-var-dep-files
$(foreach var,$(V_DEP_VARS),$(saved-var-$(var)))
endef

define imp-dep
$(if $2,$(eval $1-$2:)$1-$2)
endef

define file-imp-dep
$(if $1,$(eval $1:)$1)
endef
