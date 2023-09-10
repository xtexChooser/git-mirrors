# ========== Build ==========
BUILD_DIR ?= $(VENDOR_CODE_DIR)/out
APPLY_DIR = $(BUILD_DIR)/apply
STAMPS_DIR = $(APPLY_DIR)/stamps
STAMP_APPLICATION = $(STAMPS_DIR)/application
# reference stamp is for usage that need a time in the future
# a old timestamp as reference can be used to avoid clock skew warning by make
STAMP_REF = $(STAMPS_DIR)/reference
VARS_DIR = $(APPLY_DIR)/vars
