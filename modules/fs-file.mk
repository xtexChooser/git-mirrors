FS_FILE_VARS = V_TARGET_NAME V_POST $(v-deps-var) V_PATH V_EXIST V_CREATE V_TEMPLATE V_TPL_DEPS V_COPY V_USER V_USER_ID V_GROUP V_GROUP_ID V_ACCESS V_SYMLINK
define fs-file0
$(if $(call not,$(call is-false,$(V_EXIST))),
$(eval V_TARGET_NAME?=$(V_PATH))

$(if $(V_TEMPLATE),$(eval V_CREATE=template BACKEND=$(word 1,$(V_TEMPLATE)) SRC=$(word 2,$(V_TEMPLATE)) $(wordlist 3,$(words $(V_TEMPLATE)),$(V_TEMPLATE)))
$(eval V_TPL_DEPS += $(word 2,$(V_TEMPLATE))))
$(if $(V_COPY),$(eval V_CREATE=copy SRC=$(V_COPY))$(eval V_TPL_DEPS += $(V_COPY)))

$(call mktrace, Define exist fs-file target: $(V_UNIT))
$(call mktrace-vars,$(FS_FILE_VARS))
$(if $(V_GROUP),$(if $(V_GROUP_ID),$(call mkerr, Both V_GROUP and V_GROUP_ID is defined for $(V_PATH))))
$(if $(V_USER),$(if $(V_USER_ID),$(call mkerr, Both V_USER and V_USER_ID is defined for $(V_PATH))))

$(call apply-target,$(V_PATH))
$(if $(call strneq,$(V_TARGET_NAME),$(V_PATH)),
$(call vt-target,$(V_TARGET_NAME))
$(V_TARGET_NAME): $(V_PATH)
)
$(if $(V_USER)$(V_USER_ID)$(V_GROUP)$(V_GROUP_ID)$(V_ACCESS),$(call vt-target,$(V_PATH)))
$(V_PATH): $(v-deps) $(V_TPL_DEPS) $(call imp-dep,user,$(V_USER)) $(call imp-dep,group,$(V_GROUP)) \
		| $(v-deps-order)
	export E_MAJOR=fs-file E_PATH=$(V_PATH)
	$(if $(V_SYMLINK),
	if [[ ! -e "$(V_PATH)" || "$$$$(realpath $(V_PATH))" != "$(V_SYMLINK)" ]]; then
		$(RM) -f $(V_PATH)
		ln -s $(V_SYMLINK) $(V_PATH)
		$(call succ, Created symlink $(V_PATH))
		$(call vpost, E_MINOR=symlink)
	fi
	)
	$(if $(call not,$(V_SYMLINK)),
	if [[ ! -e $(V_PATH) $(foreach tpldep,$(V_TPL_DEPS) $(v-var-dep-files) $(V_DEPS),|| "$(tpldep)" -nt "$(V_PATH)" ) ]]; then
		$(MKDIR) -p $(dir $(V_PATH))
		$(if $(V_CREATE),
		$(MAKE) $(MAKE_FLAGS) E_MAJOR=fs-file-create E_PATH=$(V_PATH) mkfile-$(V_CREATE),
		$(call err, File $(V_PATH) is missing but V_CREATE is not defined))
		if [[ ! -e $(V_PATH) ]]; then
			$(call err, Failed to create file $(V_PATH))
		fi
		$(call succ, Created file ($(firstword $(V_CREATE))) $(V_PATH))
		$(call vpost, E_MINOR=created)
	fi
	)
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

$(call mktrace, Define non-exist fs-file target: $(V_UNIT))
$(call mktrace-vars,$(FS_FILE_VARS))

$(call apply-target,$(V_TARGET_NAME))
$(call vt-target,$(V_TARGET_NAME))
$(V_TARGET_NAME): $(v-deps)
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

$(call vt-target, mkfile-empty mkfile-download mkfile-copy mkfile-symlink)
mkfile-empty:
	touch $(E_PATH)

mkfile-download:
	if which curl $(DROP_STDOUT_ERR); then
		$(call trace, Download $(URL) to $(E_PATH) with cURL)
		curl -L -o $(E_PATH) $(URL)
	elif which wget $(DROP_STDOUT_ERR); then
		$(call trace, Download $(URL) to $(E_PATH) with wget)
		wget --max-redirect 10 -v -O $(E_PATH) $(URL)
	else
		$(call err, Neither of curl or wget is available)
	fi

mkfile-copy:
	cat $(SRC) > $(E_PATH)

mkfile-symlink:
	ln -s $(SRC) $(E_PATH)

define mkfile-run0
$(if $(TARGET),$(if $(CMD),$(call mkerr, Both TARGET and CMD is defined for $(E_PATH))))
$(if $(TARGET),mkfile-run: $(TARGET))
$(if $(CMD),mkfile-run:
	$(CMD))
endef

$(if $(call streq,$(E_MAJOR),fs-file-create),$(eval $(mkfile-run0))$(call vt-target, mkfile-run))

define mkfile-template0
mkfile-template: template
TPL_BACKEND := $(BACKEND)
TPL_IN := $(SRC)
TPL_OUT := $(E_PATH)
endef
$(if $(call streq,$(E_MAJOR),fs-file-create),$(eval $(mkfile-template0))$(call vt-target, mkfile-template))
