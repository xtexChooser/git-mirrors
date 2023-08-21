musl-alltypes-out:=$(curout)alltypes/

cflags-inc += -isystem $(musl-alltypes-out)obj/include/
$(out)/.cflags.m: $(musl-alltypes-out)obj/include/bits/alltypes.h
$(out)/.cppflags.m: $(musl-alltypes-out)obj/include/bits/alltypes.h
$(musl-alltypes-out)obj/include/bits/alltypes.h:
	$(call action,"GEN  ")
	rm -f $@
	$(MAKE) -C $(musl-alltypes-out) \
		-f $$(readlink -f $(musl-src)Makefile) \
		ARCH=$(MUSL_ARCH) \
		srcdir=$$(readlink -f $(musl-src)) \
		obj/include/bits/alltypes.h
