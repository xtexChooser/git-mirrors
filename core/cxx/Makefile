$(call load-dirs, std/)
$(call load-objs, core, cxxabi.o ssp.o start.o)

$(call load-objs, core-crti, crti0.o)
$(call load-objs, core-crtn, crtn0.o)
cflags-core-crti = $(cflags-core) -O0
cflags-core-crtn = $(cflags-core) -O0
$(curout)crti0.%: CFLAGS += -I external/musl/src/crt/$(MUSL_ARCH)/
$(curout)crtn0.%: CFLAGS += -I external/musl/src/crt/$(MUSL_ARCH)/
