ifneq "$(LEONIS_ANSI)" "false"
define ansi-color
\033[0;$(strip $(1))m
endef
ansi-clear=\033[0m
else
ansi-color=
ansi-clear=
endif

define print-ansi
$(info $(shell printf -- "$(strip $(1))$(ansi-clear)"))
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
