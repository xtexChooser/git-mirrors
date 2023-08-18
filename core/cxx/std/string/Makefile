$(call load-objs, core, musl.o)

define rule_core_cxx_std_string
$(eval $$(out)/core/cxx/std/string/musl.%: cflags += -isystem $$(musl-src)src/include/ -isystem $$(musl-src)include/)
endef

make-finalize += rule_core_cxx_std_string
