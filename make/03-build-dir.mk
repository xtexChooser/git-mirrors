$(BUILD_DIR):
	mkdir -p $(BUILD_DIR)

$(call run-on-build,$(BUILD_DIR))