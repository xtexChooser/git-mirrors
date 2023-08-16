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

define load-dir
$(eval curdirs += $1)
$(eval curdir := $1)
$(eval curout := $(out)/$1)
$(eval include $1Makefile)
$(eval curdirs := $(filter-out $(curdir),$(curdirs)))
$(eval curdir := $(lastword $(curdirs)))
endef

define load-objs
$(foreach __sub,$1,$(eval allobjs += $(out)/$(curdir)$(__sub)))
endef
define load-objs-core
$(call load-objs,$1)
$(foreach __sub,$1,$(eval allobjs-core += $(out)/$(curdir)$(__sub)))
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

include scripts/Makefile.c.mk

include arch/$(ARCH)/base.mk
