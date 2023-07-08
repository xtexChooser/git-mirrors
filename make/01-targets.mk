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
$(call trace, Loading vendor state $(VENDOR_STATES_DIR)/$(1))
include $(VENDOR_STATES_DIR)/$(1)
endef
$(call define-inline-func,load-state)
