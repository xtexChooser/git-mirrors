# Default task
default: build test

# Tools
NINJA ?= ninja
NINJA_FLAGS ?= -j8
READLINK ?= readlink -f

# Leonis paths
LEONIS_BASE_DIR ?= .
LEONIS_PLAYGROUND_DIR ?= $(LEONIS_BASE_DIR)/playground
LEONIS_MAKE_DIR ?= $(LEONIS_BASE_DIR)/make
LEONIS_STATES_DIR ?= $(LEONIS_BASE_DIR)/states
LEONIS_NINJA_DIR ?= $(LEONIS_BASE_DIR)/ninja

# Vendor paths
ifndef $(VENDOR_CODE_DIR)
$(warning You are running Leonis playground)
endif
VENDOR_CODE_DIR ?= $(LEONIS_PLAYGROUND_DIR)
BUILD_DIR ?= $(VENDOR_CODE_DIR)/build
VENDOR_MAKE_DIR ?= $(VENDOR_CODE_DIR)/make

# Include make files

LEONIS_BUILD_DEPS = $(empty)
LEONIS_TEST_DEPS = $(empty)
LEONIS_APPLY_DEPS = $(empty)

LEONIS_STATE_NINJA = $(empty)

include $(LEONIS_MAKE_DIR)/*.mk
include $(LEONIS_STATES_DIR)/*.mk
-include $(VENDOR_MAKE_DIR)/*.mk

# states.ninja
$(call run-on-build,$(BUILD_DIR)/states.ninja)

LEONIS_STATE_NINJA_MARKER = $(BUILD_DIR)/states.marker.$(shell echo "$(LEONIS_STATE_NINJA)" | sha256sum | head -c6)
$(BUILD_DIR)/states.ninja: $(LEONIS_STATE_NINJA_MARKER)
	printf "$(LEONIS_STATE_NINJA)" | sed -e 's/^ //' > $(BUILD_DIR)/states.ninja

$(LEONIS_STATE_NINJA_MARKER):
	rm -f $(BUILD_DIR)/states.marker.*
	touch $(LEONIS_STATE_NINJA_MARKER)

# Builtin tasks

.PHONY: default build test apply

build: $(LEONIS_BUILD_DEPS)

test: build $(LEONIS_TEST_DEPS)

apply: test $(LEONIS_APPLY_DEPS)
	$(NINJA) $(NINJA_FLAGS) -C $(BUILD_DIR)
