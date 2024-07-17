# ========================= Users =========================

USERADD = useradd
USERMOD = usermod
USERDEL = userdel -r
USER_PASSWD_FILES := /etc/passwd $$([[ -e /etc/shadow ]] && echo /etc/shadow || true)

USER_VARS = V_TARGET_NAME V_NAME V_POST $(v-deps-var) V_EXIST V_UID V_GID V_SYSTEM \
	V_HOME_DIR V_EXPIRE V_INACTIVE V_GROUPS V_NOLOGINIT V_NON_UNIQUE V_PASSWORD V_SHELL \
	V_USERGROUP
define user0
$(eval V_TARGET_NAME?=user-$(V_NAME))
$(if $(V_PASSWORD),$(call mkwarn, Password for user $(V_NAME) is defined in makefile. This is insecure!))

$(call mktrace, Define shadow user target: $(V_NAME))
$(call mktrace-vars,$(USER_VARS))
$(call apply-target,$(V_TARGET_NAME))
$(call vt-target,$(V_TARGET_NAME))
$(V_TARGET_NAME): $(v-deps) \
	$(if $(V_GROUPS),$(call empty-rules,$(addprefix group-,$(V_GROUPS)))) \
	$(if $(V_GID),$(call empty-rules,group-$(V_GID))) \
	$(if $(V_SHELL),$(call empty-rules,$(V_SHELL))) \
	| $(v-deps-order)
	export E_MAJOR=user E_NAME=$(V_NAME)
$(if $(call is-true,$(V_EXIST)),
	if ! grep -E '^$(V_NAME):' $(USER_PASSWD_FILES) $(DROP_STDOUT); then
		$(USERADD) $(if $(V_UID),--uid $(V_UID)) $(if $(V_GID),--gid $(V_GID)) $(if $(V_SYSTEM),--system) \
			$(if $(V_HOME_DIR),--home-dir $(V_HOME_DIR)) $(if $(V_EXPIRE),--expiredate $(V_EXPIRE)) \
			$(if $(V_INACTIVE),--inactive $(V_INACTIVE)) \
			$(if $(V_GROUPS),--groups $(subst $(space),$(comma),$(V_GROUPS))) \
			$(if $(call is-true,$(V_NOLOGINIT)),--no-log-init) $(if $(call is-true,$(V_NON_UNIQUE)),--non-unique) \
			$(if $(V_PASSWORD),--password "$(V_PASSWORD)") \
			$(if $(V_SHELL),--shell "$(V_SHELL)") \
			$(if $(V_USERGROUP),--user-group)
			$(V_NAME)
		$(call succ, Created user $(V_NAME))
		$(call vpost, E_MINOR=created)
	fi
)
$(if $(call is-false,$(V_EXIST)),
	if grep -E '^$(V_NAME):' $(USER_PASSWD_FILES) $(DROP_STDOUT); then
		$(USERDEL) $(E_NAME)
		$(call succ, Deleted user $(V_NAME))
		$(call vpost, E_MINOR=deleted)
	fi
)
	$(if $(V_UID),[[ "$$(id -u $(V_NAME))" != "$(V_UID)" ]] && $(USERMOD) --uid $(V_UID) $(V_NAME) && $(call succ, Updated UID for user $(V_NAME) to $(V_UID)) && $(call vpost, E_MINOR=updated-uid))
	$(if $(V_GID),[[ "$$(id -g $(V_NAME))" != "$(V_GID)" ]] && [[ "$$(id -ng $(V_NAME))" != "$(V_GID)" ]] && $(USERMOD) --gid $(V_GID) $(V_NAME) && $(call succ, Updated GID for user $(V_NAME) to $(V_GID)) && $(call vpost, E_MINOR=updated-gid))
	$(if $(V_GROUPS),$(foreach grp,$(V_GROUPS),id -nG $(V_NAME) || ! grep -E '\b$(grp)\b' && $(USERMOD) --append --groups $(grp) $(V_NAME) && $(call succ, Add group $(grp) to user $(V_NAME)) && $(call vpost, E_MINOR=updated-groups);))
	$(if $(V_SHELL),[[ "$$(grep -F "$(V_NAME)" /etc/passwd | cut -d: -f7)" != "$(V_SHELL)" ]] && $(USERMOD) --shell $(V_SHELL) $(V_NAME) && $(call succ, Updated shell for user $(V_NAME) to $(V_SHELL)) && $(call vpost, E_MINOR=updated-shell))

$(call unset-vars)
endef

$(call define-func, user)

$(call vt-target, user-create user-delete)
user-create:
	$(USERADD) $(E_NAME)
	$(call succ, Created user $(E_NAME))

user-delete:
	$(USERDEL) $(E_NAME)
	$(call succ, Deleted user $(E_NAME))

# ========================= Groups =========================

GROUPADD = groupadd
GROUPMOD = groupmod
GROUPDEL = groupdel
GROUP_PASSWD_FILES := /etc/group $$([[ -e /etc/gshadow ]] && echo /etc/gshadow || true)

GROUP_VARS = V_TARGET_NAME V_NAME V_POST $(v-deps-var) V_EXIST V_GID V_SYSTEM \
	V_USERS V_NON_UNIQUE V_PASSWORD
define group0
$(eval V_TARGET_NAME?=group-$(V_NAME))
$(if $(V_PASSWORD),$(call mkwarn, Password for group $(V_NAME) is defined in makefile. This is insecure!))

$(call mktrace, Define shadow group target: $(V_NAME))
$(call mktrace-vars,$(GROUP_VARS))
$(call apply-target,$(V_TARGET_NAME))
$(call vt-target,$(V_TARGET_NAME))
$(V_TARGET_NAME): $(v-deps) \
	$(if $(V_USERS),$(call empty-rules,$(addprefix user-,$(V_GROUPS))))
	export E_MAJOR=group E_NAME=$(V_NAME)
$(if $(call is-true,$(V_EXIST)),
	if ! grep -E '^$(V_NAME):' $(GROUP_PASSWD_FILES) $(DROP_STDOUT); then
		$(GROUPADD) $(if $(V_GID),--gid $(V_GID)) $(if $(V_SYSTEM),--system) \
			$(if $(call is-true,$(V_NON_UNIQUE)),--non-unique) \
			$(if $(V_PASSWORD),--password "$(V_PASSWORD)") \
			$(V_NAME)
		$(call succ, Created group $(V_NAME))
		$(call vpost, E_MINOR=created)
	fi
)
$(if $(call is-false,$(V_EXIST)),
	if grep -E '^$(V_NAME):' $(GROUP_PASSWD_FILES) $(DROP_STDOUT); then
		$(GROUPDEL) $(E_NAME)
		$(call succ, Deleted group $(V_NAME))
		$(call vpost, E_MINOR=deleted)
	fi
)
	$(if $(V_GID),[[ "$$(grep -E '^$(V_NAME):' /etc/group | cut -d: -f3)" != "$(V_GID)" ]] && $(GROUPMOD) --gid $(V_GID) $(V_NAME) && $(call succ, Updated GID for group $(V_NAME) to $(V_GID)) && $(call vpost, E_MINOR=updated-gid))
	$(if $(V_USERS),$(foreach usr,$(V_USERS),id -nG $(usr) || ! grep -E '\b$(V_NAME)\b' && $(GROUPMOD) --append --users $(usr) $(V_NAME) && $(call succ, Add user $(usr) to group $(V_NAME)) && $(call vpost, E_MINOR=updated-groups);))

$(call unset-vars)
endef

$(call define-func, group)

$(call vt-target, group-create group-delete)
group-create:
	$(GROUPADD) $(E_NAME)
	$(call succ, Created group $(E_NAME))

group-delete:
	$(GROUPDEL) $(E_NAME)
	$(call succ, Deleted group $(E_NAME))

