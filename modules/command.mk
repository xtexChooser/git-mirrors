CMD_SHELL ?= bash -c

CMD_VARS=V_TARGET_NAME V_NAME V_SHELL V_POST V_DEPS V_CMD V_APPLY
define cmd0
$(eval V_TARGET_NAME?=cmd-$(V_NAME))
$(eval V_SHELL ?= $(CMD_SHELL))
$(if $(V_NAME),,$(error V_NAME is not defined))

$(call mktrace,Define command target: $(V_NAME))
$(call mktrace-vars,$(CMD_VARS))
$(if $(V_APPLY),$(call apply-target,$(V_TARGET_NAME)))
$(call vt-target,$(V_TARGET_NAME))

$(V_TARGET_NAME): $(V_DEPS)
	export E_MAJOR=cmd E_NAME=$(V_NAME)
	$(CMD_SHELL) $(V_APPLY)
	$(call succ, Executed command $(V_CMD))
	$(call vpost, E_MINOR=run)

$(call unset-vars)
endef

$(call define-func, cmd)

$(call vt-target,cmd-run)
cmd-run:
	$(CMD_SHELL) $(E_CMD)
	$(call succ, Executed $(E_CMD))
