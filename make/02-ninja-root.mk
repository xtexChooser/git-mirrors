$(call run-on-build,$(BUILD_DIR)/build.ninja)

$(BUILD_DIR)/build.ninja: $(LEONIS_NINJA_DIR)/build.ninja
	cp $(LEONIS_NINJA_DIR)/build.ninja $(BUILD_DIR)/build.ninja
