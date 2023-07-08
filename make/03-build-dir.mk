$(BUILD_DIR):
	@mkdir -p $@
	$(call succ, Created $@)

$(call run-on-build,$(BUILD_DIR))

$(APPLY_DIR): $(BUILD_DIR)
	@mkdir $@
	$(call succ, Created $@)

$(call run-on-apply,$(APPLY_DIR))

$(APPLY_TIME_FILE): $(APPLY_DIR)
	@touch $@
	$(call trace, Updated $@)

$(call run-on-apply,$(APPLY_TIME_FILE))
