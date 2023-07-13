HOST_USER = $(USER)
$(if $(HOSTNAME),,$(eval HOSTNAME = $(shell hostname)))
HOST_KERNEL = $(shell uname -a)
