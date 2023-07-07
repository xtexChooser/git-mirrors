define apply-target
$(eval APPLY_TARGETS += $(1))
endef

define target-group0
$(info $(1)  -  $(2))
target-group-$(1)+=$(2)
$(1): $$(target-group-$(1))
endef
$(call define-inline-func,target-group)

define add-to-target-group0
target-group-$(1)+=$(2)
endef
$(call define-inline-func,add-to-target-group)
$(call define-inline-func,add-to-tgt-group,add-to-target-group0)
