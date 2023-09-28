define implict-rules
$(eval $1:
.PHONY: $1)
endef

define implict-deps
$(call implict-rules,$1)$1
endef
