PODMAN = podman

PODMAN_IMAGE_VARS = V_TARGET_NAME V_NAME V_IMAGE V_POST $(v-deps-var) V_EXIST V_LATEST
define podman-image0
$(eval V_NAME?=$(subst :,-,$(V_IMAGE)))
$(eval V_TARGET_NAME?=podman-image-$(V_NAME))
$(if $(findstring :,$(V_IMAGE)),,$(call mkerr, V_IMAGE $(V_IMAGE) should include the tag, e.g. :latest))

$(call mktrace, Define podman-image target: $(V_NAME))
$(call mktrace-vars,$(PODMAN_IMAGE_VARS))

$(call apply-target,$(V_TARGET_NAME))
$(call vt-target,$(V_TARGET_NAME))
$(V_TARGET_NAME): $(v-deps) | $(v-deps-order)
	export E_MAJOR=podman_image E_IMAGE=$(V_IMAGE)
$(if $(call not,$(call is-false,$(V_EXIST))),
	if ( ! $(PODMAN) image exists $(V_IMAGE) ) || $(if $(call is-true,$(V_LATEST)),true,false) ; then
		$(PODMAN) image pull $(V_IMAGE)
		if ! $(PODMAN) image exists $(V_IMAGE); then
			$(call err, Podman says image $(V_IMAGE) is pulled but it is still not found)
		fi
		$(call vpost, E_MINOR=pulled)
		$(call succ, Pulled podman image $(V_IMAGE))
	fi
)
$(if $(call is-false,$(V_EXIST)),
	if $(PODMAN) image exists $(V_IMAGE); then
		$(PODMAN) image rm $(V_IMAGE)
		if $(PODMAN) image exists $(V_IMAGE); then
			$(call err, Podman says image $(V_IMAGE) is removed but it is still found)
		fi
		$(call vpost, E_MINOR=removed)
		$(call succ, Removed podman image $(V_IMAGE))
	fi
)

$(call unset-vars)
endef

$(call define-func, podman-image)
