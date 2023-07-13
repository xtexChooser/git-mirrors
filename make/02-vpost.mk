define vpost
$(if $(V_POST),$(1) $(MAKE) $(MAKE_FLAGS) $(V_POST)
	$(call succ, Called post-func $(V_POST)))
endef
