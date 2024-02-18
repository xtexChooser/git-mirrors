$(call define-mkdir-target,$(BUILD_DIR),)
$(call define-mkdir-target,$(APPLY_DIR),)
$(call define-mkdir-target,$(STAMPS_DIR),)
$(call define-mkdir-target,$(VARS_DIR),)
$(call define-touch-target,$(STAMP_APPLICATION),$(STAMPS_DIR)/.dir,,1)
$(call define-touch-target,$(STAMP_REF),$(STAMPS_DIR)/.dir,-d -365days,1)

$(call run-on-apply,$(BUILD_DIR) $(APPLY_DIR) $(STAMP_APPLICATION) $(STAMP_REF))
