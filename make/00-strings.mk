define starts-with
$(if $(patsubst $(1)%,,$(2)),,y)
endef

define ends-with
$(if $(patsubst %$(1),,$(2)),,y)
endef
