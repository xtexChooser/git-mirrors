HOST_USER = $(USER)
$(if $(call not,$(HOSTNAME)),$(eval HOSTNAME := $(shell hostname)))
HOST_KERNEL ?= $(shell uname -a)

HOME := $(HOME)/
$(if $(call not,$(HOME)),$(eval HOME := $(shell echo $$HOME))/)
