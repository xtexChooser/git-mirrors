$(call load-dirs, std/)
$(call load-objs, core, atexit.o ssp.o start.o crti0.o crtn0.o)
$(curout)crti0.%: CFLAGS += -I external/musl/src/crt/$(MUSL_ARCH)/
$(curout)crtn0.%: CFLAGS += -I external/musl/src/crt/$(MUSL_ARCH)/
