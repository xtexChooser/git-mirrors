# ========================= Variables =========================
LEONIS_BUILD_DEPS = $(empty)
LEONIS_APPLY_DEPS = $(empty)
APPLY_TARGETS ?= $(empty)

# ========================= Paths =========================
ifndef LEONIS_BASE_DIR
$(call mkerr, LEONIS_BASE_DIR is not specified)
endif
LEONIS_MAKE_DIR ?= $(LEONIS_BASE_DIR)/make
LEONIS_MODULES_DIR ?= $(LEONIS_BASE_DIR)/modules
LEONIS_CONTRIB_DIR ?= $(LEONIS_BASE_DIR)/contrib
LEONIS_EXTERNAL_DIR ?= $(LEONIS_BASE_DIR)/external

VENDOR_CODE_DIR ?= .
VENDOR_MAKE_DIR ?= $(VENDOR_CODE_DIR)/make
VENDOR_MODULES_DIR ?= $(VENDOR_CODE_DIR)/modules
STATES_DIR ?= $(VENDOR_CODE_DIR)/states

# ========================= Modules =========================
include $(LEONIS_MAKE_DIR)/*.mk
include $(LEONIS_MODULES_DIR)/*.mk
include $(VENDOR_MAKE_DIR)/*.mk
-include $(VENDOR_MODULES_DIR)/*.mk
$(call end-all)

# ========================= Finalization =========================
$(export-all)
$(call-deferred-fns)

# ========================= Core tasks =========================
$(call vt-target, default build apply fmt)

build: $(LEONIS_BUILD_DEPS)

CUSTOM_APPLY ?= $(empty)
define default-apply
$(if $(APPLY_TARGETS),,$(call mkerr, APPLY_TARGETS is empty))
apply: build $(LEONIS_APPLY_DEPS)
	@$(MAKE) $(MAKE_JOBSERVER_FLAGS) $(MAKE_FLAGS) $(if $T,$T,$(APPLY_TARGETS))
endef
$(if $(CUSTOM_APPLY),,$(eval $(call default-apply)))

fmt:
	@$(LEONIS_CONTRIB_DIR)/fmt
