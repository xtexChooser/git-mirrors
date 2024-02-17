X_CONTAINER_SERVICE_VARS = V_TARGET_NAME V_SERVICE V_STOPPED V_ARGS V_POST $(v-deps-var)

define x-container-service0
$(eval V_PIDFILE?=/var/run/containers/$(V_SERVICE).pid)

$(call mktrace, Define x-container-service target: $(V_SERVICE))
$(call mktrace-vars,$(X_CONTAINER_SERVICE_VARS))

$(DINITD_DIR)/$(V_SERVICE): $(VENDOR_MODULES_DIR)/containers.mk
	@cat >$$@ <<EOF
	type = bgprocess
	command = atre apply x-service-$(V_SERVICE)-start
	stop-command = atre apply x-service-$(V_SERVICE)-stop
	pid-file = $(V_PIDFILE)
	restart = true
	EOF

$(call vt-target,x-service-$(V_SERVICE)-start x-service-$(V_SERVICE)-stop)
x-service-$(V_SERVICE)-start:
	$(PODMAN) container run --name $(V_SERVICE) --rm -d --pidfile=$(V_PIDFILE) --replace \
		$(V_ARGS)

x-service-$(V_SERVICE)-stop:
	$(PODMAN) container rm -f -i $(V_SERVICE)

$$(call dinit-service)
V_SERVICE	= $(V_SERVICE)
V_RUNNING	= $(call not,$(V_STOPPED))
$$(call end)

$(call unset-vars)
endef
$(call define-func, x-container-service)
