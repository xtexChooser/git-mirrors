HOST_USER ?= $(USER)
$(if $(call not,$(HOSTNAME)),$(eval HOSTNAME := $(shell hostname)))
HOST_KERNEL ?= $(shell uname -a)

HOME := $(abspath $(HOME))
$(if $(call not,$(HOME)),$(eval HOME := $$(abspath $$(shell echo $$$$HOME))))
