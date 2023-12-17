v-deps-var := V_DEPS V_DEP_VARS

define v-deps
$(V_DEPS) $(call dep-vars,$(V_DEP_VARS))
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
