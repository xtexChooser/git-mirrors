# ========================= Variables =========================
LEONIS_APPLY_DEPS = $(empty)
APPLY_TARGETS ?= $(empty)

# ========================= Paths =========================
ifndef LEONIS_BASE_DIR
$(error LEONIS_BASE_DIR is not specified)
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
-include $(VENDOR_MAKE_DIR)/*.mk

ifneq ($(LEONIS_ONLY_LOAD),)

include $(wildcard $(LEONIS_ONLY_LOAD) \
	$(LEONIS_MODULES_DIR)/$(LEONIS_ONLY_LOAD) \
	$(VENDOR_MODULES_DIR)/$(LEONIS_ONLY_LOAD))

else

ifeq ($(LEONIS_LOAD_ALL),)
# ========================= Core task wrappers =========================
$(call vt-target, default apply fmt)
CUSTOM_APPLY ?= $(empty)

ifeq ($(CUSTOM_APPLY),)
apply:
	@$(MAKE) LEONIS_LOAD_ALL=y $(MAKE_JOBSERVER_FLAGS) $(MAKE_FLAGS) apply
endif

fmt:
	@$(LEONIS_CONTRIB_DIR)/fmt "$(LEONIS_CONTRIB_DIR)"

else

include $(LEONIS_MODULES_DIR)/*.mk
-include $(VENDOR_MODULES_DIR)/*.mk
$(call vendor-targets)
$(call load-requested-states)
$(call end-all)

# ========================= Finalization =========================
$(export-all)
$(call-deferred-fns)

# ========================= Core tasks =========================
$(call vt-target, default apply)
CUSTOM_APPLY ?= $(empty)

ifeq ($(CUSTOM_APPLY),)
$(if $(APPLY_TARGETS),,$(call mkerr, APPLY_TARGETS is empty))
apply: $(LEONIS_APPLY_DEPS)
	@$(MAKE) $(if $T,$T,$(APPLY_TARGETS))
endif

endif
endif
