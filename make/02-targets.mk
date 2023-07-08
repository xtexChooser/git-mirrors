define apply-target
$(eval APPLY_TARGETS += $(1))
endef

define virtual-target
$(eval .PHONY: $(1))
endef

define vt-target
$(call virtual-target,$(1))
endef

define load-state0
$(call mktrace, Loading vendor state $(1))
LEONIS_LOADED_STATES+=$(1)
include $(VENDOR_STATES_DIR)/$(1)
endef
$(call define-inline-func,load-state)

define check-loaded
$(findstring $(1),$(LEONIS_LOADED_STATES))
endef
