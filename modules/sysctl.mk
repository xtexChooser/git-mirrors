$(call vt-target, sysctl-reload)
sysctl-reload:
	sysctl --system -w
	$(call succ, Reloaded sysctl configurations)
