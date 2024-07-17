FS_DIRECTORY_VARS = V_TARGET_NAME V_POST $(v-deps-var) V_PATH V_EXIST V_USER V_USER_ID V_GROUP V_GROUP_ID V_RECURSIVE V_ACCESS
define fs-directory0
$(if $(call not,$(call is-false,$(V_EXIST))),
$(eval V_TARGET_NAME?=$(V_PATH))

$(call mktrace, Define exist fs-directory target: $(V_UNIT))
$(call mktrace-vars,$(FS_DIRECTORY_VARS))
$(if $(V_GROUP),$(if $(V_GROUP_ID),$(call mkerr, Both V_GROUP and V_GROUP_ID is defined for $(V_PATH))))
$(if $(V_USER),$(if $(V_USER_ID),$(call mkerr, Both V_USER and V_USER_ID is defined for $(V_PATH))))

$(call apply-target,$(V_PATH))
$(if $(call strneq,$(V_TARGET_NAME),$(V_PATH)),
$(call vt-target,$(V_TARGET_NAME))
$(V_TARGET_NAME): $(V_PATH)
)
$(if $(V_USER)$(V_USER_ID)$(V_GROUP)$(V_GROUP_ID)$(V_ACCESS),$(call vt-target,$(V_PATH)))
$(V_PATH): $(v-deps) $(call imp-dep,user,$(V_USER)) $(call imp-dep,group,$(V_GROUP)) \
		| $(v-deps-order)
	export E_MAJOR=fs-directory E_PATH=$(V_PATH)
	if [[ ! -e $(V_PATH) ]]; then
		$(MKDIR) -p $(V_PATH)
		$(call succ, Created directory $(V_PATH))
		$(call vpost, E_MINOR=created)
	fi
	$(if $(V_USER)$(V_USER_ID),
	if [[ "$$$$(stat -c "%$(if $(V_USER),U,u)" $(V_PATH))" != "$(V_USER)$(V_USER_ID)" ]]; then
		$(CHOWN) $(if $(call is-true,$(V_RECURSIVE)),--recursive) $(V_USER)$(V_USER_ID):$$(stat -c "%g" $(V_PATH)) $(V_PATH)
		$(call succ, Updated the owner of $(V_PATH) to $(V_USER)$(V_USER_ID))
		$(call vpost, E_MINOR=chown)
	fi
	)
	$(if $(V_GROUP)$(V_GROUP_ID),
	if [[ "$$$$(stat -c "%$(if $(V_GROUP),G,g)" $(V_PATH))" != "$(V_GROUP)$(V_GROUP_ID)" ]]; then
		$(CHOWN) $(if $(call is-true,$(V_RECURSIVE)),--recursive) $$(stat -c "%u" $(V_PATH)):$(V_GROUP)$(V_GROUP_ID) $(V_PATH)
		$(call succ, Updated the group of $(V_PATH) to $(V_GROUP)$(V_GROUP_ID))
		$(call vpost, E_MINOR=chgrp)
	fi
	)
	$(if $(V_ACCESS),
	if [[ "$$$$(stat -c "%#a" $(V_PATH))" != "$(V_ACCESS)" ]]; then
		$(CHMOD) $(if $(call is-true,$(V_RECURSIVE)),--recursive) $(V_ACCESS) $(V_PATH)
		$(call succ, Updated the access of $(V_PATH) to $(V_ACCESS))
		$(call vpost, E_MINOR=chmod)
	fi
	)
,
$(eval V_TARGET_NAME?=dir-$(V_PATH))

$(call mktrace, Define non-exist fs-directory target: $(V_UNIT))
$(call mktrace-vars,$(FS_DIRECTORY_VARS))

$(call apply-target,$(V_TARGET_NAME))
$(call vt-target,$(V_TARGET_NAME))
$(V_TARGET_NAME): $(v-deps)
	export E_MAJOR=fs-directory E_PATH=$(V_PATH)
	if [[ -e $(V_PATH) ]]; then
		$(RM) -rf $(V_PATH)
		$(call succ, Removed directory $(V_PATH))
		$(call vpost, E_MINOR=removed)
	fi
)

$(call unset-vars)
endef

$(call define-func, fs-directory)

define add-fs-directory
$(call fs-directory)
$(eval V_PATH = $1)
$(eval V_EXIST = y)
$(call end)
endef
