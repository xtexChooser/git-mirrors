# .EXPORT_ALL_VARIABLES leaks too many macros

define should-export
$(call not,$(subst A,,$(subst B,,$(subst C,,$(subst D,,$(subst E,,$(subst F,,$(subst G,,$(subst H,,$(subst I,,$(subst J,,$(subst K,,$(subst L,,$(subst M,,$(subst N,,$(subst O,,$(subst P,,$(subst Q,,$(subst R,,$(subst S,,$(subst T,,$(subst U,,$(subst V,,$(subst W,,$(subst X,,$(subst Y,,$(subst Z,,$(subst 0,,$(subst 1,,$(subst 2,,$(subst 3,,$(subst 4,,$(subst 5,,$(subst 6,,$(subst 7,,$(subst 8,,$(subst 9,,$(subst _,,$1))))))))))))))))))))))))))))))))))))))
endef

define export-all
$(foreach var,$(.VARIABLES),$(if $(call streq,$(origin $(var)),override)$(call streq,$(origin $(var)),file),$(if $(call should-export,$(var)),$(if $(LEONIS_TRACE_EXPORT),$(info export $(var)))$(eval export $(var)))))
endef
