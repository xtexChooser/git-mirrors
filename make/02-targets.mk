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
$(if $(leonis-requested-states-loaded),$$(call load-state-now,$1))
)
endef
$(call define-inline-func,load-state)

define is-state-requested
$(filter $1,$(leonis-requested-states))
endef

define load-requested-states
$(eval leonis-requested-states-loaded:=y)
$(foreach state,$(leonis-requested-states),$(eval -include $(STATES_DIR)/$(state)/pre.mk))
$(foreach state,$(leonis-requested-states),$(call load-state-now,$(state)))
$(foreach state,$(leonis-requested-states),$(eval -include $(STATES_DIR)/$(state)/post.mk))
endef

define load-state-now0
$(call mktrace, Loading vendor state $1)
leonis-loaded-states += $1
$(empty)-include $(STATES_DIR)/$1.mk
$(empty)-include $(STATES_DIR)/$1/base.mk
endef
$(call define-inline-func,load-state-now)

define is-state-loaded
$(filter $1,$(leonis-loaded-states))
endef

define invoke-hooks
$(call mktrace, Calling state hook $1)
$(foreach state,$(leonis-requested-states),-include $(STATES_DIR)/$(state)/hooks/$1.mk)
endef
