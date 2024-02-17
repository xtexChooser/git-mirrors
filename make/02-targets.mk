define apply-target
$(eval APPLY_TARGETS += $1)
endef

define vt-target
$(eval .PHONY: $1)
endef

define cmd-target
$(eval .PHONY: $1
$1:
	@$2)
endef

define delete-on-err
$(eval .DELETE_ON_ERROR: $1)
endef

define load-state0
$(if $(call check-loaded,$1),,
$(call mktrace, Loading vendor state $1)
leonis-loaded-states += $1
$(empty)-include $(STATES_DIR)/$1.mk
$(empty)-include $(STATES_DIR)/$1/base.mk
)
endef
$(call define-inline-func,load-state)

define check-loaded
$(filter $1,$(leonis-loaded-states))
endef
