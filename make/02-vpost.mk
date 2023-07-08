define call-vpost
$(if $(V_POST),$(1) $(MAKE) $(MAKE_FLAGS) $(V_POST))
endef
