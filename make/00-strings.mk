define starts-with
$(if $(patsubst $(1)%,,$(2)),,y)
endef

define ends-with
$(if $(patsubst %$(1),,$(2)),,y)
endef

define is-true
$(findstring y,$(1))$(findstring t,$(1))$(findstring 1,$(1))
endef

define is-false
$(findstring n,$(1))$(findstring f,$(1))$(findstring 0,$(1))
endef
