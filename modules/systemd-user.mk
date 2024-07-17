LOGINCTL = loginctl

LOGINCTL_VARS = V_TARGET_NAME V_USER V_LINGER V_POST $(v-deps-var)
define loginctl0
$(eval V_TARGET_NAME?=loginctl-$(V_USER))

$(call mktrace, Define loginctl target: $(V_USER))
$(call mktrace-vars,$(LOGINCTL_VARS))
$(call apply-target,$(V_TARGET_NAME))
$(call vt-target,$(V_TARGET_NAME))

$(V_TARGET_NAME): $(v-deps) $(call imp-dep,user,$(V_USER)) \
	| $(v-deps-order)
	export E_MAJOR=loginctl E_USER=$(V_USER)
$(if $(call is-true,$(V_LINGER)),
	if [[ ! -e "/var/lib/systemd/linger/$(V_USER)" ]]; then
		$(LOGINCTL) enable-linger $(V_USER)
		$(call succ, Enabled SD linger state for $(V_USER))
		$(call vpost, E_MINOR=linger-enabled)
	fi
)
$(if $(call is-false,$(V_LINGER)),
	if [[ -e "/var/lib/systemd/linger/$(V_USER)" ]]; then
		$(LOGINCTL) disable-linger $(V_USER)
		$(call succ, Disabled SD linger state for $(V_USER))
		$(call vpost, E_MINOR=linger-disabled)
	fi
)

$(call unset-vars)
endef

$(call define-func, loginctl)

$(call vt-target, loginctl-enable-linger loginctl-disable-linger)
loginctl-enable-linger:
	$(LOGINCTL) enable-linger $(E_USER)
	$(call succ, Enabled SD linger state for $(V_USER))
loginctl-disable-linger:
	$(LOGINCTL) disable-linger $(E_USER)
	$(call succ, Disabled SD linger state for $(V_USER))

