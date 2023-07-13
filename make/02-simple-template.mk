define define-mkdir-target0
$(1): $(2)
	@$$(MKDIR) -p $$@
	$$(call succ, Created $$@)
endef

$(call define-inline-func,define-mkdir-target)

define define-touch-target0
$(1): $(2)
	@$$(TOUCH) $(3) $$@
	$(if $(4),,$$(call succ, Touched $$@))
endef

$(call define-inline-func,define-touch-target)
