DINITCTL = dinitctl
DINITCTL_SYSTEM = dinitctl --system
DINITCTL_DEPS := $(call imp-dep,pkg,dinit) $(call imp-dep,systemd,dinit.service)
DINITD_DIR ?= /etc/dinit.d
DINITD_USER_DIR ?= $(HOME)/.config/dinit.d

DINIT_SERVICE_VARS = V_TARGET_NAME V_SERVICE V_RUNNING V_SYSTEM V_DINITCTL V_POST $(v-deps-var)
define dinit-service0
$(eval V_TARGET_NAME?=dinit-$(V_SERVICE))
$(eval V_DINITCTL ?= $(if $(V_SYSTEM),$(DINITCTL_SYSTEM),$(DINITCTL)))

$(call mktrace, Define dinit service target: $(V_SERVICE))
$(call mktrace-vars,$(DINIT_SERVICE_VARS))
$(call apply-target,$(V_TARGET_NAME))
$(call vt-target,$(V_TARGET_NAME))
$(V_TARGET_NAME): $(DINITCTL_DEPS) $(v-deps) \
		$(if $(V_SYSTEM)$(call streq,root,$(USER)),$(call file-imp-dep,$(DINITD_DIR)/$(V_SERVICE)) \
		$(call file-imp-dep,/lib/dinit.d/$(V_SERVICE)) \
		$(call file-imp-dep,/run/dinit.d/$(V_SERVICE)) \
		$(call file-imp-dep,/usr/local/lib/dinit.d/$(V_SERVICE)), \
		$(call file-imp-dep,$(DINITD_USER_DIR)/$(V_SERVICE))) \
		| $(v-deps-order)
	export E_MAJOR=dinit E_SERVICE=$(V_SERVICE)
$(if $(call is-true,$(V_RUNNING)),
	if ! $(V_DINITCTL) is-started $(V_SERVICE) $(DROP_STDOUT); then
		$(V_DINITCTL) start $(V_SERVICE)
		$(call succ, Started dinit service $(V_SERVICE))
		$(call vpost, E_MINOR=started)
	fi
)
$(if $(call is-false,$(V_RUNNING)),
	if $(V_DINITCTL) is-started $(V_SERVICE) $(DROP_STDOUT); then
		$(V_DINITCTL) stop $(V_SERVICE)
		$(call succ, Stopped dinit service $(V_SERVICE))
		$(call vpost, E_MINOR=stopped)
	fi
)

$(call unset-vars)
endef

$(call define-func, dinit-service)

$(call vt-target, dinit-start dinit-stop dinit-restart dinit-reload dinit-shutdown)
dinit-start: $(DINITCTL_DEPS)
	$(DINITCTL) start $(E_SERVICE)
	$(call succ, Started dinit service $(E_SERVICE))
dinit-stop: $(DINITCTL_DEPS)
	$(DINITCTL) stop $(E_SERVICE)
	$(call succ, Stopped dinit service $(E_SERVICE))
dinit-restart: $(DINITCTL_DEPS)
	$(DINITCTL) restart $(E_SERVICE)
	$(call succ, Restarted dinit service $(E_SERVICE))
dinit-reload: $(DINITCTL_DEPS)
	$(DINITCTL) reload $(E_SERVICE)
	$(call succ, Reloaded dinit service $(E_SERVICE))
dinit-shutdown: $(DINITCTL_DEPS)
	$(DINITCTL) shutdown
	$(call succ, Shutting down dinit daemon)
