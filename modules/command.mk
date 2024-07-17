CMD_SHELL ?= bash -c

CMD_VARS = V_TARGET_NAME V_NAME V_POST $(v-deps-var) V_CMD V_APPLY
define cmd0
$(eval V_TARGET_NAME?=cmd-$(V_NAME))
$(if $(V_NAME),,$(call mkerr, V_NAME is not defined))

$(call mktrace, Define command target: $(V_NAME))
$(call mktrace-vars,$(CMD_VARS))
$(if $(V_APPLY),$(call apply-target,$(V_TARGET_NAME)))
$(call vt-target,$(V_TARGET_NAME))

$(V_TARGET_NAME): $(v-deps)
	export E_MAJOR=cmd E_NAME=$(V_NAME)
	$(V_CMD)
	$(call succ, Executed command $(V_CMD))
	$(call vpost, E_MINOR=run)

$(call unset-vars)
endef
$(call define-func, cmd)

CMD_STAMP_VARS = V_TARGET_NAME V_NAME V_POST $(v-deps-var) V_CMD V_APPLY V_PATH
define cmd-stamp0
$(eval V_TARGET_NAME?=cmd-stamp-$(V_NAME))
$(eval V_PATH?=$(STAMPS_DIR)/cmd-stamp-$(V_NAME))
$(if $(V_NAME),,$(call mkerr, V_NAME is not defined))

$(call mktrace, Define command-stamped target: $(V_NAME))
$(call mktrace-vars,$(CMD_STAMP_VARS))
$(if $(V_APPLY),$(call apply-target,$(V_TARGET_NAME)))
$(call vt-target,$(V_TARGET_NAME))

$(V_TARGET_NAME): $(V_PATH)

$(call apply-target,$(V_PATH))
$(V_PATH): $(v-deps) | $(v-deps-order)
	export E_MAJOR=cmd-stamp E_NAME=$(V_NAME)
	$(V_CMD)
	$(TOUCH) $$@
	$(call succ, Executed command $(V_CMD))
	$(call vpost, E_MINOR=run)

$(call unset-vars)
endef
$(call define-func, cmd-stamp)

$(call vt-target, cmd-run)
cmd-run:
	$(CMD_SHELL) $(E_CMD)
	$(call succ, Executed $(E_CMD))
