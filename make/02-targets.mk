define apply-target
$(eval APPLY_TARGETS += $1)
endef

define virtual-target
$(eval .PHONY: $1)
endef

define vt-target
$(call virtual-target,$1)
endef

define delete-on-err
$(eval .DELETE_ON_ERROR: $1)
endef

define load-state0
$(call mktrace, Loading vendor state $1)
LEONIS_LOADED_STATES += $1
-include $(STATES_DIR)/$1.mk include $(STATES_DIR)/$1/base.mk
endef
$(call define-inline-func,load-state)

define check-loaded
$(findstring $1,$(LEONIS_LOADED_STATES))
endef
