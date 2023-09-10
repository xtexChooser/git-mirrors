define template-backend-bash-tpl
if ! source <($(SHELL) $(LEONIS_EXTERNAL_DIR)/bash-tpl $(BASH_TPL_FLAGS) $(1)) > $(2); then
	$(SHELL) $(LEONIS_EXTERNAL_DIR)/bash-tpl $(BASH_TPL_FLAGS) $(1)
	$(RM) -f $(2)
	fi
endef

MO_FLAGS += --fail-on-function --allow-function-arguments --fail-on-file
define template-backend-mo
$(SHELL) $(LEONIS_EXTERNAL_DIR)/mo $(MO_FLAGS) $(1) > $(2)
endef

define template0
template:
	$(call template-backend-$(TPL_BACKEND),$(TPL_IN),$(TPL_OUT))
	$(call succ,Template ($(TPL_BACKEND)) from $(TPL_IN) to $(TPL_OUT))
endef
$(if $(TPL_BACKEND),$(eval $(template0)$(call vt-target,template0)))
