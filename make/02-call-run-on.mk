define run-on-build
$(eval LEONIS_BUILD_DEPS += $1)
endef

define run-on-apply
$(eval LEONIS_APPLY_DEPS += $1)
endef
