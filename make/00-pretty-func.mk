# ========== Variable Functions ==========
define define-func
$(eval $(call define-func0,$(1),$(if $(2),$(2),$(1)0)))
endef

define define-func0
$ define $(1)
$$(call set-end-function,$(2))
$ endef
endef

define set-end-function
$(eval end-function = $(1) $(end-function))
endef

define end
$(eval $(call $(firstword $(end-function))))
$(eval end-function=$(wordlist 2,$(words $(end-function)),$(end-function)))
endef

define end-all
$(foreach fn,$(end-function),$(eval $(call $(fn))))
endef

# ========== Inline Functions ==========
define define-inline-func
$(eval $(call define-inline-func0,$(1),$(if $(2),$(2),$(1)0)))
endef

define define-inline-func0
$ define $(1)
$$(eval $$(call $(2),$$(strip $$(1)),$$(strip $$(2)),$$(strip $$(3)),$$(strip $$(4)),$$(strip $$(5)),$$(strip $$(6)),$$(strip $$(7)),$$(strip $$(8)),$$(strip $$(9)),$$(strip $$(10))))
$ endef
endef

# ========== Utilities Functions ==========
define unset-vars
$(foreach var,$(0),$(eval undefine $(var)))
endef
