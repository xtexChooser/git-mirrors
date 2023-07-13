$(call define-mkdir-target,$(BUILD_DIR),)
$(call define-mkdir-target,$(APPLY_DIR),$(BUILD_DIR))
$(call define-mkdir-target,$(STAMPS_DIR),$(APPLY_DIR))
$(call define-touch-target,$(STAMP_APPLICATION),$(STAMPS_DIR))

$(call run-on-apply,$(BUILD_DIR) $(APPLY_DIR) $(STAMP_APPLICATION))
