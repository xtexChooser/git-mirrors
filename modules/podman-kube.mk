PODMAN = podman
PODMAN_FILTER_KUBE = sed -e 's/^  creationTimestamp: .*//g' -e 's/^$#.*//g'

PODMAN_KUBE_VARS = V_TARGET_NAME V_NAME V_POST $(v-deps-var) V_PATH V_POD V_FILE V_RUNNING
define podman-kube0
$(eval V_TARGET_NAME?=podman-kube-$(V_NAME))
$(eval V_PATH?=$(VARS_DIR)/podman-kube-$(V_NAME).sha1sum)

$(call mktrace, Define podman-kube target: $(V_NAME))
$(call mktrace-vars,$(PODMAN_KUBE_VARS))

$(call apply-target,$(V_TARGET_NAME))
$(call vt-target,$(V_TARGET_NAME) $(V_PATH))
$(V_TARGET_NAME): $(V_PATH)

$(V_PATH): $(V_FILE) $(v-deps) | $(v-deps-order)
	export E_MAJOR=podman_kube E_NAME=$(V_NAME) E_PATH=$(V_PATH) E_POD=$(V_POD)
	if ! ($(PODMAN) pod exists $(V_POD) && [[ -e $(V_PATH) ]]); then
		$(PODMAN) kube play --replace $(V_FILE) $(DROP_STDOUT)
		if ! $(PODMAN) pod exists $(V_POD); then
			$(call err, Loaded podman kube config from $(V_FILE) but pod $(V_POD) is not created)
		fi
		$(PODMAN) kube generate --type pod $(V_POD) | $(PODMAN_FILTER_KUBE) | sha1sum > $(V_PATH)
		$(call vpost, E_MINOR=created)
		$(call succ, Created podman pod $(V_POD))
	elif [[ "$(V_FILE)" -nt "$(V_PATH)" || "$$$$(<$(V_PATH))" != "$$$$($(PODMAN) kube generate --type pod $(V_POD) | $(PODMAN_FILTER_KUBE) | sha1sum)" ]]; then
		$(PODMAN) kube play --replace $(V_FILE) $(DROP_STDOUT)
		$(PODMAN) kube generate --type pod $(V_POD) | $(PODMAN_FILTER_KUBE) | sha1sum > $(V_PATH)
		$(call vpost, E_MINOR=updated)
		$(call succ, Updated podman pod $(V_POD))
	fi
$(if $(call not,$(call is-false,$(V_RUNNING))),
	if [[ "$$$$(podman pod inspect $(V_POD) -f "{{ .State }}")" != "Running" ]]; then
		$(PODMAN) pod start $(V_POD) $(DROP_STDOUT)
		$(TOUCH) $$@
		$(call vpost, E_MINOR=started)
		$(call succ, Started podman pod $(V_POD))
	fi,
	if [[ "$$$$(podman pod inspect $(V_POD) -f "{{ .State }}")" == "Running" ]]; then
		$(PODMAN) pod stop $(V_POD) $(DROP_STDOUT)
		$(call vpost, E_MINOR=stopped)
		$(call succ, Stopped podman pod $(V_POD))
	fi
)
	if [[ '' != '' $(foreach depfile,$(V_DEPS) $(v-var-dep-files),|| "$(depfile)" -nt "$(V_PATH)" ) ]]; then
		$(PODMAN) pod restart $(V_POD) $(DROP_STDOUT)
		$(call vpost, E_MINOR=restart)
		$(call succ, Restarted podman pod $(V_POD))
	fi
	$(TOUCH) $$@

$(call unset-vars)
endef

$(call define-func, podman-kube)
