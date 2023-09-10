define defer
$(eval deffered-fn-stack += $(1))
endef

define call-deferred-fns
$(foreach fn,$(deffered-fn-stack),$(eval $(call $(fn))))
endef
