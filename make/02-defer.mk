define defer
$(eval deffered-fn-stack += $1)
endef

define call-deferred-fns
$(foreach fn,$(deffered-fn-stack),$(if $(LEONIS_TRACE_DEFFERED_FN),$(info Calling deffered func $(fn)))$(eval $(call $(fn))))
endef

define defer-deps0
$(empty)define defer-deps-$1-impl
$$$$(eval $1: $$$$($2))
$(empty)endif
$(empty)endef
$(call defer,defer-deps-$1-impl)
endef
$(call define-inline-func,defer-deps)
