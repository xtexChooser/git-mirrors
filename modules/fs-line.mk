FS_LINE_VARS = V_TARGET_NAME V_NAME V_POST $(v-deps-var) V_PATH V_FLAGS V_MATCH V_LINE V_PREPEND
define fs-line0
$(eval V_TARGET_NAME?=fs-line-$(V_PATH)-$(V_NAME))
$(eval V_LINE:=$(subst $$,$$$$$$$$$$$$$$$$,$(subst ','"'"',$(V_LINE))))
$(eval V_MATCH:=$(subst $$,$$$$$$$$$$$$$$$$,$(subst ','"'"',$(V_MATCH))))

$(call mktrace, Define exist fs-line target: $(V_UNIT))
$(call mktrace-vars,$(FS_LINE_VARS))

$(if $(call streq,$(V_TARGET_NAME),$(V_PATH)),$(call err, fs-line target name ($(V_TARGET_NAME)) can't be the same as V_PATH))

$(call apply-target,$(V_TARGET_NAME))
$(call vt-target,$(V_TARGET_NAME))

$(V_TARGET_NAME): $(if $(call not,$(findstring no-dep,$(V_FLAGS))$(findstring append,$(V_FLAGS))),$(V_PATH)) $(v-deps) \
		| $(v-deps-order)
	export E_MAJOR=fs-line E_NAME=$(V_NAME) E_PATH=$(V_PATH)
	if [[ -e $(V_PATH) ]]; then
		if ! grep -F '$(V_LINE)' $(V_PATH) $(DROP_STDOUT); then
			if $(if $(V_MATCH),grep -E '$(V_MATCH)' $(V_PATH) $(DROP_STDOUT), false); then
				$(MV) $(V_PATH) $(V_PATH).bak
				$(SED) -E -e 's/$(V_MATCH)/$(V_LINE)/g' $(V_PATH).bak > $(V_PATH)
				$(call succ, Replaced line with '$(V_LINE)' in $(V_PATH))
				$(call vpost, E_MINOR=replaced)
			else
				$(CP) $(V_PATH) $(V_PATH).bak
				$(if $(call is-true,$(V_PREPEND)),{ echo '$(V_LINE)'; cat $(V_PATH); } > $(V_PATH).newtmp; mv $(V_PATH).newtmp $(V_PATH),echo 'V_LINE' >> $(V_PATH))
				$(call succ, $(if $(call is-true,$(V_PREPEND)),Prepended,Appended) line '$(V_LINE)' to $(V_PATH))
				$(call vpost, E_MINOR=appended)
			fi
		fi
	else
		$(if $(findstring append,$(V_FLAGS)),
		echo 'V_LINE' > $(V_PATH)
		$(call succ, Appended line '$(V_LINE)' to $(V_PATH))
		$(call vpost, E_MINOR=appended),
		$(call err, fs-line target file $(V_PATH) not found))
	fi

$(call unset-vars)
endef

$(call define-func, fs-line)
