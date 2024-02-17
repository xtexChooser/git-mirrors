X_CONTAINER_SERVICE_VARS = V_TARGET_NAME V_SERVICE V_STOPPED V_ARGS V_POST $(v-deps-var)

define x-container-service0
$(eval V_PIDFILE?=/var/run/containers/$(V_SERVICE).pid)
$(eval V_DEP_VARS+=$(addprefix x-container-$(V_SERVICE)-,args start-cmd stop-cmd))
$(eval x-container-$(V_SERVICE)-args:=$(V_ARGS))
$(eval x-container-$(V_SERVICE)-start-cmd:=$(PODMAN) container run \
	--name $(V_SERVICE) --rm -d --pidfile=$(V_PIDFILE) --replace \
	--hostname=$(V_SERVICE) \
	$(V_ARGS))
$(eval x-container-$(V_SERVICE)-stop-cmd:=$(PODMAN) container rm -f -i $(V_SERVICE); \
	rm -rf $(V_PIDFILE))

$(call mktrace, Define x-container-service target: $(V_SERVICE))
$(call mktrace-vars,$(X_CONTAINER_SERVICE_VARS))

$(DINITD_DIR)/$(V_SERVICE): $(v-deps) $(VENDOR_MODULES_DIR)/containers.mk
	@cat >$$@ <<EOF
	type = bgprocess
	command = $(x-container-$(V_SERVICE)-start-cmd)
	stop-command = $(x-container-$(V_SERVICE)-stop-cmd)
	pid-file = $(V_PIDFILE)
	restart = true
	EOF
	$(DINITCTL) stop --force --ignore-unstarted $(V_SERVICE)
	$(DINITCTL) reload $(V_SERVICE)

$$(call dinit-service)
V_SERVICE	= $(V_SERVICE)
V_RUNNING	= $(call not,$(V_STOPPED))
V_DEPS		+= $(DINITD_DIR)/$(V_SERVICE)
$$(call end)

$$(call fs-file)
V_PATH		= $$(DINITD_DIR)/boot.d/$(V_SERVICE)
$(if $(call not,$(call is-true,$(V_STOPPED))),V_SYMLINK = ../$(V_SERVICE),V_EXIST = n)
$$(call end)

$(call unset-vars)
endef
$(call define-func, x-container-service)
