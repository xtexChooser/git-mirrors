.DEFAULT_GOAL := default
default:

.PHONY: force
force: ;

comma := ,
empty :=
space := $(empty) $(empty)

define empty-rules
$(eval $1:)$1
endef

define require-variable
ifeq ($($1),)
$(error Variable $1 is required but not available)
endif
endef
