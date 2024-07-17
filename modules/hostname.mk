HOSTNAME_ETC = /etc/hostname
HOSTNAMECTL = /usr/bin/hostnamectl

HOSTNAME_VARS = V_TARGET_NAME V_POST $(v-deps-var) V_HOSTNAME V_PRETTYNAME
define hostname0
$(eval V_TARGET_NAME?=hostname)
$(if $(V_HOSTNAME),,$(call mkerr, V_HOSTNAME is not defined))

$(call mktrace, Define hostname target: $(V_TARGET_NAME))
$(call mktrace-vars,$(HOSTNAME_VARS))
$(if $(call streq,$(V_HOSTNAME),$(HOSTNAME)),,$(call apply-target,$(V_TARGET_NAME)))
$(call vt-target,$(V_TARGET_NAME))

$(V_TARGET_NAME): $(v-deps) | $(v-deps-order)
	export E_MAJOR=hostname E_HOSTNAME=$(V_HOSTNAME) E_PRETTYNAME=$(V_PRETTYNAME)
	$(MAKE) $(MAKE_FLAGS) hostname-set
	$(call vpost, E_MINOR=run)

$(call unset-vars)
endef

$(call define-func, hostname)

$(call vt-target, hostname-set)
hostname-set:
	if [[ -e $(HOSTNAME_ETC) ]]; then
		echo $(E_HOSTNAME) > $(HOSTNAME_ETC)
		$(call succ, Wrote hostname $(E_HOSTNAME) to $(HOSTNAME_ETC))
	fi
	if [[ -e $(HOSTNAMECTL) ]]; then
		$(HOSTNAMECTL) hostname --static $(E_HOSTNAME)
		$(call succ, Updated hostname to $(E_HOSTNAME) with hostnamectl)
		$(if $(E_PRETTYNAME),$(HOSTNAMECTL) hostname --pretty $(E_PRETTYNAME)
		$(call succ, Updated pretty hostname to $(E_PRETTYNAME) with hostnamectl))
	fi
