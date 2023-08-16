musl-src:=$(curdir)src/
musl-out:=$(curout)
musl-inc:=$(musl-src)include/

$(curout)config.mak: $(curdir)src/.git
	$(call action,"GEN  ")
	SRCTREE=$$(readlink -e $(musl-src))/
	cd $(musl-out); $${SRCTREE}configure \
		AR=llvm-ar RANLIB=llvm-ranlib \
		CC="$(CC) --target=$(MUSL_TARGET)" \
		CFLAGS="-O3 -fuse-ld=lld" \
		LDFLAGS="-Wl,--build-id=sha1 ../../core/utils/arith/arith.a" \
		CROSS_COMPILE=$(MUSL_TARGET) \
		--srcdir=../../../external/musl/src \
		--prefix=/usr \
		--enable-optimize \
		--target=$(MUSL_TARGET)

$(curout)override.mk: $(curout)config.mak
	$(call action,"GEN  ")
	grep 'ARCH =' $< | sed 's/^/override /g' > $@

$(curout)lib/libc.so: $(curout)config.mak $(curout)override.mk $(compiler-rt-builtins-lib)
	$(call action,"MAKE ")
	$(MAKE) -C $(musl-out) -f override.mk -f Makefile

default: $(curout)lib/libc.so
