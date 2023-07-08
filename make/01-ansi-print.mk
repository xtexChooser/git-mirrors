ifneq "$(LEONIS_ANSI)" "false"
define ansi-color
\033[0;$(strip $(1))m
endef
ansi-clear=\033[0m
else
ansi-color=
ansi-clear=
endif

# ========== Print in Makefiles ==========

define mkprint-ansi
$(info $(shell printf -- "$(strip $(1))$(ansi-clear)"))
endef

define mkprintc
$(call mkprint-ansi, $(call ansi-color, $(1))$(2))
endef

define mktrace
$(if $(findstring true,$(LEONIS_TRACE)),$(call mkprintc,30,- $(strip $(1))))
endef

define mktrace-vars
$(if $(findstring true,$(LEONIS_TRACE)),$(if $(findstring false,$(LEONIS_TRACE_VARS)),,$(foreach var,$(1),$(call mkprintc,30,   - $(var)=$(value $(var))))))
endef

define mklog
$(if $(findstring false,$(LEONIS_PRINT_INFO)),,$(call mkprintc,37,- $(strip $(1))))
endef

define mksucc
$(if $(findstring false,$(LEONIS_PRINT_INFO)),,$(call mkprintc,32,- $(strip $(1))))
endef

# ========== Print in Recipes ==========

define print-ansi
printf -- "$(strip $(1))$(ansi-clear)\n"
endef

define printc
$(call print-ansi, $(call ansi-color, $(1))$(2))
endef

define trace
$(if $(findstring true,$(LEONIS_TRACE)),$(call printc,30,- $(strip $(1))))
endef

define log
$(if $(findstring false,$(LEONIS_PRINT_INFO)),,$(call printc,37,- $(strip $(1))))
endef

define succ
$(if $(findstring false,$(LEONIS_PRINT_INFO)),,$(call printc,32,- $(strip $(1))))
endef
