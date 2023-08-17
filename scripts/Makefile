.ONESHELL:
SHELL:=bash

mkparent	= @mkdir -p $$(dirname $@)

comma:=,

.DELETE_ON_ERROR:
.PHONY: default
default:

ifeq ($(ARCH),)
$(error ARCH is not specified)
endif
include arch/$(ARCH)/config.mk

-include local.config.mk

define load-dir
$(eval curdirs += $1)
$(eval curdir := $1)
$(eval curout := $(out)/$1)
$(eval include $1Makefile)
$(eval curdirs := $(filter-out $(curdir),$(curdirs)))
$(eval curdir := $(lastword $(curdirs)))
endef

define load-objs
$(foreach __sub,$2,$(eval allobjs += $(out)/$(curdir)$(__sub))
$(eval allobjs-$(strip $1) += $(out)/$(curdir)$(__sub))
$(eval $(out)/$(curdir)$(__sub): OBJ_GROUP=$(strip $1)))
endef
define load-dirs
$(foreach __sub,$1,$(call load-dir,$(curdir)$(__sub)))
endef

ifeq ($V$(VERBOSE),)
Q:=@
else
Q:=
endif

define action
$Q$(mkparent)
echo $1"$(if $2,$2,$@)"
endef

define saved
$(eval $(strip $1).m:
	$$(call action,"SAVE ")
	echo "saved_$(strip $2)=$3" > $$@

$(strip $2)_file=$(strip $1).m
define saved_restore_$(strip $2)
$$(eval
include $(strip $1).m
ifneq ($$$$(saved_$(strip $2)),$3)
$$$$(shell rm -f $(strip $1).m)
endif
)
endef
make-finalize += saved_restore_$(strip $2)
)
endef

include scripts/Makefile.c.mk

include arch/$(ARCH)/base.mk

$(call load-dirs,core/ arch/$(ARCH)/ external/ doc/)

.PHONY: clean
clean:
	$(call action,"RM   ","$(out)/*")
	rm -rf $(out)/*

$(foreach __finalize,$(make-finalize),$(call $(__finalize)))
