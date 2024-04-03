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
$(if $(call is-state-requested,$1),,
$(call mktrace, Requesting vendor state $1)
leonis-requested-states += $1
)
endef
$(call define-inline-func,load-state)

define is-state-requested
$(filter $1,$(leonis-requested-states))
endef

define load-requested-states
$(foreach state,$(leonis-requested-states),$(call load-state-now,$(state)))
endef

define load-state-now0
$(if $(call is-state-loaded,$1),,
$(call mktrace, Loading vendor state $1)
leonis-loaded-states += $1
$(empty)-include $(STATES_DIR)/$1.mk
$(empty)-include $(STATES_DIR)/$1/base.mk
)
endef
$(call define-inline-func,load-state-now)

define is-state-loaded
$(filter $1,$(leonis-loaded-states))
endef
