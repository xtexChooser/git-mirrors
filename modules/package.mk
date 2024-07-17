PKCON = pkcon -y --plain

PACKAGE_VARS = V_TARGET_NAME V_PKCON V_POST $(v-deps-var) V_PKG V_INSTALLED V_INST_FILE
define package0
$(eval V_TARGET_NAME?=pkg-$(V_PKG))
$(eval V_PKCON ?= $(PKCON))

$(call mktrace, Define package target: $(V_UNIT))
$(call mktrace-vars,$(PACKAGE_VARS))
$(call apply-target,$(V_TARGET_NAME))
$(call vt-target,$(V_TARGET_NAME))
$(V_TARGET_NAME): $(v-deps) $(if $(call is-true,$(V_INSTALLED)),$(V_INST_FILE)) \
		| $(v-deps-order)
	export E_MAJOR=pkg E_PKG=$(V_PKG)
$(if $(call is-true,$(V_INSTALLED)),$(if $(call not,$(V_INST_FILE)),
	if ! $(V_PKCON) resolve $(V_PKG) | grep 'installed' $(DROP_STDOUT); then
		$(V_PKCON) install $(V_PKG)
		$(call succ, Installed package $(V_UNIT))
		$(call vpost, E_MINOR=installed)
	fi
))
$(if $(call is-false,$(V_INSTALLED)),
$(if $(V_INST_FILE),
	if [[ -e $(V_INST_FILE) ]]; then,
	if $(V_PKCON) resolve $(V_PKG) | grep 'installed' $(DROP_STDOUT); then
)
		$(V_PKCON) remove $(V_PKG)
		$(call succ, Removed package $(V_UNIT))
		$(call vpost, E_MINOR=removed)
	fi
)

$(if $(V_INST_FILE),$(if $(call is-true,$(V_INSTALLED)),
$(V_INST_FILE): $(v-deps)
	export E_MAJOR=pkg E_PKG=$(E_PKG)
	$(V_PKCON) install $(V_PKG)
	$(call succ, Installed package $(V_PKG))
	$(call vpost, E_MINOR=installed)
))

$(call unset-vars)
endef

$(call define-func, package)

$(call vt-target, pkg-refresh pkg-update)
pkg-refresh:
	$(PKCON) refresh
	$(call succ, Refreshed package cache)

pkg-update:
	$(PKCON) update $(E_PKG)
	$(call succ, Updated package $(E_PKG))
