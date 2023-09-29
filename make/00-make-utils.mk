.DEFAULT_GOAL := default
default: build test

.PHONY: force
force: ;

comma := ,
empty :=
space := $(empty) $(empty)

define empty-rules
$(eval $1:)$1
endef
