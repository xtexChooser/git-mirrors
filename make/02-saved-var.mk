define save-var0
$(eval saved-var-$1:=$(VARS_DIR)/$1.txt)
$(saved-var-$1): $(VARS_DIR)
	$$(file >$$@,saved_var_$1=$$($2))
	@$(call trace,Saved variable cache $1)

define saved-var-$1-restore
include $(saved-var-$1)
ifneq ($$$$(saved_var_$1),$$($2))
$$$$(shell $(RM) -f $(saved-var-$1))
$$$$(call mktrace,Invalidated variable cache $1)
endif
endef
$(call defer,saved-var-$1-restore)
endef
$(call define-inline-func,save-var)
