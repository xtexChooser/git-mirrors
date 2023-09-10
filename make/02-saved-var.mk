define save-var0
$(if $(saved-var-$1),$(if $(call strneq,$(saved-var-$1),$(VARS_DIR)/$1.txt),
$(error Saved var $1 is already defined with different path: $(saved-var-$1))),
$(eval $(call save-var1,$1,$2)))
endef

define save-var1
$(eval saved-var-$1:=$(VARS_DIR)/$1.txt)
$(saved-var-$1): $(VARS_DIR)
	$$(file >$$@,saved_var_$1=$$($2))
	@$(call trace,Saved variable cache $1)

define saved-var-$1-restore
include $(saved-var-$1)
ifneq ($$$$(saved_var_$1),$$($2))
$$$$(shell $(RM) -f $(saved-var-$1))
saved_var_$1_changed=1
$$$$(call mktrace,Invalidated variable cache $1)
endif
endef
$(call defer,saved-var-$1-restore)
endef

$(call define-inline-func,save-var)

define dep-vars
$(foreach var,$1,$(call save-var,dep-var-$(var),$(var))$(saved-var-$(var)))
endef
