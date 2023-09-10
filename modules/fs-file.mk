FS_FILE_VARS=V_TARGET_NAME V_POST V_DEPS V_PATH V_EXIST V_CREATE V_USER V_USER_ID V_GROUP V_GROUP_ID V_ACCESS
define fs-file0
$(if $(call not,$(call is-false,$(V_EXIST))),
$(eval V_TARGET_NAME?=$(V_PATH))

$(call mktrace,Define exist fs-file target: $(V_UNIT))
$(call mktrace-vars,$(FS_FILE_VARS))
$(if $(V_GROUP),$(if $(V_GROUP_ID),$(error Both V_GROUP and V_GROUP_ID is defined for $(V_PATH))))
$(if $(V_USER),$(if $(V_USER_ID),$(error Both V_USER and V_USER_ID is defined for $(V_PATH))))
$(if $(call not,$(V_CREATE)),$(error V_CREATE is not defined for $(V_PATH). Please specific the way to create file when not found))

$(call apply-target,$(V_PATH))
$(if $(call strneq,$(V_TARGET_NAME),$(V_PATH)),
$(call vt-target,$(V_TARGET_NAME))
$(V_TARGET_NAME): $(V_PATH)
)
$(if $(V_USER)$(V_USER_ID)$(V_GROUP)$(V_GROUP_ID)$(V_ACCESS),$(call vt-target,$(V_PATH)))
$(V_PATH): $(V_DEPS)
	export E_MAJOR=fs-file E_PATH=$(V_PATH)
	if [[ ! -e $(V_PATH) ]]; then
		$(MKDIR) -p $(dir $(V_PATH))
		$(MAKE) $(MAKE_FLAGS) E_PATH=$(V_PATH) mkfile-$(V_CREATE)
		if [[ ! -e $(V_PATH) ]]; then
			$(call err, Failed to create file $(V_PATH))
		fi
		$(call succ, Created file ($(firstword $(V_CREATE))) $(V_PATH))
		$(call vpost, E_MINOR=created)
	fi
	$(if $(V_USER)$(V_USER_ID),
	if [[ "$$$$(stat -c "%$(if $(V_USER),U,u)" $(V_PATH))" != "$(V_USER)$(V_USER_ID)" ]]; then
		$(CHOWN) $(V_USER)$(V_USER_ID):$$(stat -c "%g" $(V_PATH)) $(V_PATH)
		$(call succ, Updated the owner of $(V_PATH) to $(V_USER)$(V_USER_ID))
		$(call vpost, E_MINOR=chown)
	fi
	)
	$(if $(V_GROUP)$(V_GROUP_ID),
	if [[ "$$$$(stat -c "%$(if $(V_GROUP),G,g)" $(V_PATH))" != "$(V_GROUP)$(V_GROUP_ID)" ]]; then
		$(CHOWN) $$(stat -c "%u" $(V_PATH)):$(V_GROUP)$(V_GROUP_ID) $(V_PATH)
		$(call succ, Updated the group of $(V_PATH) to $(V_GROUP)$(V_GROUP_ID))
		$(call vpost, E_MINOR=chgrp)
	fi
	)
	$(if $(V_ACCESS),
	if [[ "$$$$(stat -c "%#a" $(V_PATH))" != "$(V_ACCESS)" ]]; then
		$(CHMOD) $(V_ACCESS) $(V_PATH)
		$(call succ, Updated the access of $(V_PATH) to $(V_ACCESS))
		$(call vpost, E_MINOR=chmod)
	fi
	)
,
$(eval V_TARGET_NAME?=file-$(V_PATH))

$(call mktrace,Define non-exist fs-file target: $(V_UNIT))
$(call mktrace-vars,$(FS_FILE_VARS))

$(call apply-target,$(V_TARGET_NAME))
$(call vt-target,$(V_TARGET_NAME))
$(V_TARGET_NAME): $(V_DEPS)
	export E_MAJOR=fs-file E_PATH=$(V_PATH)
	if [[ -e $(V_PATH) ]]; then
		$(RM) -f $(V_PATH)
		$(call succ, Removed file $(V_PATH))
		$(call vpost, E_MINOR=removed)
	fi
)

$(call unset-vars)
endef

$(call define-func, fs-file)

mkfile-empty:
	$(TOUCH) $(E_PATH)
