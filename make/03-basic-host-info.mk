HOST_USER = $(USER)
$(if $(call not,$(HOSTNAME)),$(eval HOSTNAME := $(shell hostname)))
$(if $(call not,$(HOST_KERNEL)),$(eval HOST_KERNEL := $(shell uname -a)))
export HOST_KERNEL
