# ========== Build ==========
BUILD_DIR ?= $(VENDOR_CODE_DIR)/out
APPLY_DIR = $(BUILD_DIR)/apply
STAMPS_DIR = $(APPLY_DIR)/stamps
STAMP_APPLICATION = $(STAMPS_DIR)/application
STAMP_REF = $(STAMPS_DIR)/reference

# ========== Exports ==========
export LEONIS_BASE_DIR
export LEONIS_MAKE_DIR
export LEONIS_MODULES_DIR

export VENDOR_CODE_DIR
export VENDOR_MAKE_DIR
export VENDOR_MODULES_DIR
export VENDOR_STATES_DIR
export BUILD_DIR
export APPLY_DIR
export APPLY_TIME_FILE
