define defer
$(eval deffered-fn-stack += $1)
endef

define call-deferred-fns
$(foreach fn,$(deffered-fn-stack),$(if $(LEONIS_TRACE_DEFFERED_FN),$(info Calling deffered func $(fn)))$(eval $(call $(fn))))
endef
