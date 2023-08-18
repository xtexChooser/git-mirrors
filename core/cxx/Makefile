$(call load-dirs, std/)
#$(call load-objs, core, atexit.o ssp.o start.o _crti.o _crtn.o)
$(call load-objs, core, atexit.o ssp.o start.o _crti.o _crtn.o)
$(curout)_crti.%: CFLAGS += -I external/musl/src/crt/$(MUSL_ARCH)/
$(curout)_crtn.%: CFLAGS += -I external/musl/src/crt/$(MUSL_ARCH)/
