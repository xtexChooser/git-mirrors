# ========== Build ==========
BUILD_DIR ?= $(VENDOR_CODE_DIR)/out
APPLY_DIR = $(VENDOR_CODE_DIR)/out/apply
APPLY_TIME_FILE = $(VENDOR_CODE_DIR)/out/apply/latest-ts

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
