HOST_USER ?= $(USER)
$(if $(call not,$(HOSTNAME)),$(eval HOSTNAME := $(shell hostname 2>/dev/null || cat /etc/hostname 2>/dev/null)))
HOST_KERNEL ?= $(shell uname -a)

HOME := $(abspath $(HOME))
$(if $(call not,$(HOME)),$(eval HOME := $$(abspath $$(shell echo $$$$HOME))))
