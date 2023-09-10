HOST_USER = $(USER)
$(if $(call not,$(HOSTNAME)),$(eval HOSTNAME := $(shell hostname)))
HOST_KERNEL ?= $(shell uname -a)
