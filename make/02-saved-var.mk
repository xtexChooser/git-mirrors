define save-var0
$(if $(saved-var-$1),$(if $(call strneq,$(saved-var-$1),$(VARS_DIR)/$1.txt),
$(error Saved var $1 is already defined with different path: $(saved-var-$1))),
$(eval $(call save-var1,$1,$2)))
endef

define save-var1
$(eval saved-var-$1:=$(VARS_DIR)/$1.txt)
$(saved-var-$1): $(VARS_DIR)
	@echo "saved_var_$1_value=$$($2)" > $$@

define saved-var-$1-restore
-include $(saved-var-$1)
ifneq ($$$$(saved_var_$1_value),$$$$($2))
$$$$(shell echo "saved_var_$1_value=$$$$($2)" > $$$$(saved-var-$1))
$$$$(call mksucc,Updated variable cache for $1)
$$$$(call mktrace,Invalidated variable cache $1. Old: $$$$(saved_var_$1_value) New: $$$$($2))
saved_var_$1_changed=1
saved_var_$1_value=$$$$($2)
endif
endef
$(call defer,saved-var-$1-restore)
endef

$(call define-inline-func,save-var)

define dep-vars
$(foreach var,$1,$(call save-var,$(var),$(var))$(saved-var-$(var)))
endef
