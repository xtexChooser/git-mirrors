# ========== Variable Functions ==========
define define-func
$(eval $(call define-func0,$(strip $(1)),$(strip $(if $(2),$(2),$(1)0))))
endef

define define-func0
$(if $(value $(1)),$(error Define function $(1) with impl $(2) but wrapper macro is defined))
$(if $(value $(2)),,$(error Define function $(1) but impl $(2) is not implemented))
$ define $(1)
$$(call push-fn-stack,$(2))
$ endef
endef

define push-fn-stack
$(eval fn-stack = $(1) $(fn-stack))
endef

define end
$(eval $(call $(firstword $(fn-stack))))
$(eval fn-stack=$(wordlist 2,$(words $(fn-stack)),$(fn-stack)))
endef

define end-all
$(foreach fn,$(fn-stack),$(eval $(call $(fn))))
endef

# ========== Inline Functions ==========
define define-inline-func
$(eval $(call define-inline-func0,$(strip $(1)),$(strip $(if $(2),$(2),$(1)0))))
endef

define define-inline-func0
$(if $(value $(1)),$(error Define inline function $(1) with impl $(2) but wrapper macro is defined))
$(if $(value $(2)),,$(error Define inline function $(1) but impl $(2) is not implemented))
$ define $(1)
$$(eval $$(call $(2),$$(strip $$(1)),$$(strip $$(2)),$$(strip $$(3)),$$(strip $$(4)),$$(strip $$(5)),$$(strip $$(6)),$$(strip $$(7)),$$(strip $$(8)),$$(strip $$(9)),$$(strip $$(10))))
$ endef
endef

# ========== Utilities Functions ==========
define unset-vars
$(foreach var,$(.VARIABLES),$(if $(call starts-with,V_,$(var)),$(eval undefine $(var))))
endef
