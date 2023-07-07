# Default task
default: build test

# Tools
MAKE_FLAGS = -j4

# Variables
LEONIS_BUILD_DEPS = $(empty)
LEONIS_TEST_DEPS = $(empty)
LEONIS_APPLY_DEPS = $(empty)
APPLY_TARGETS ?= $(empty)

# Leonis paths
ifndef LEONIS_BASE_DIR
$(error LEONIS_BASE_DIR is not specified)
endif
LEONIS_MAKE_DIR ?= $(LEONIS_BASE_DIR)/make
LEONIS_STATES_DIR ?= $(LEONIS_BASE_DIR)/states

# Vendor paths
VENDOR_CODE_DIR ?= .
BUILD_DIR ?= $(VENDOR_CODE_DIR)/out
VENDOR_MAKE_DIR ?= $(VENDOR_CODE_DIR)/make
VENDOR_STATES_DIR ?= $(VENDOR_CODE_DIR)/states

# Include make files
include $(LEONIS_MAKE_DIR)/*.mk
include $(LEONIS_STATES_DIR)/*.mk
include $(VENDOR_MAKE_DIR)/*.mk
include $(LEONIS_STATES_DIR)/*.mk
$(call end-all)

# Core tasks

.PHONY: default build test apply

build: $(LEONIS_BUILD_DEPS)

test: build $(LEONIS_TEST_DEPS)

CUSTOM_APPLY ?= $(empty)
define default-apply
$(if $(APPLY_TARGETS),,$(error APPLY_TARGETS is empty))
apply: test $(LEONIS_APPLY_DEPS)
	$(MAKE) $(MAKE_FLAGS) $(APPLY_TARGETS)
endef
$(if $(CUSTOM_APPLY),,$(eval $(call default-apply)))
