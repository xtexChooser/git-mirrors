PKCON = pkcon -y --plain

PACKAGE_VARS=V_TARGET_NAME V_PKCON V_POST V_DEPS V_PKG V_INSTALLED V_REMOVED V_INST_FILE
define package0
$(eval V_TARGET_NAME?=pkg-$(V_PKG))
$(eval V_PKCON ?= $(PKCON))
$(if $(V_INSTALLED),$(if $(V_REMOVED),$(error Both V_INSTALLED and V_REMOVED is defined for $(V_PKG))))

$(call mktrace,Define package target: $(V_UNIT))
$(call mktrace-vars,$(PACKAGE_VARS))
$(call apply-target,$(V_TARGET_NAME))
$(call vt-target,$(V_TARGET_NAME))
$(V_TARGET_NAME): $(V_DEPS) $(if $(V_REMOVED),,$(V_INST_FILE))
	export E_MAJOR=pkg E_PKG=$(E_PKG)
$(if $(V_ENABLED),
	if ! $(V_PKCON) resolve $(V_PKG) | grep 'installed:' > /dev/null; then
		$(V_PKCON) install $(V_PKG)
		$(call succ, Installed package $(V_UNIT))
		$(call vpost, E_MINOR=installed)
	fi
)
$(if $(V_REMOVED),
$(if $(V_INST_FILE),
	if [[ -e $(V_INST_FILE) ]]; then,
	if $(V_PKCON) resolve $(V_PKG) | grep 'installed:' > /dev/null; then
)
		$(V_PKCON) remove $(V_PKG)
		$(call succ, Removed package $(V_UNIT))
		$(call vpost, E_MINOR=removed)
	fi
)

$(if $(V_INST_FILE),$(if $(V_REMOVED),,
$(V_INST_FILE): $(V_DEPS)
	export E_MAJOR=pkg E_PKG=$(E_PKG)
	$(V_PKCON) install $(V_PKG)
	$(call succ, Installed package $(V_PKG))
	$(call vpost, E_MINOR=installed)
))

$(call unset-vars)
endef

$(call define-func, package)
