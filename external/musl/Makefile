musl-src:=$(curdir)src/
musl-out-base:=$(curout)
musl-out:=$(curout)out/
musl-inc:=$(musl-src)include/

$(musl-out)config.mak: $(curdir)src/.git
	$(call action,"GEN  ")
	SRCTREE=$$(readlink -e $(musl-src))/
	cd $(musl-out); $${SRCTREE}configure \
		AR=llvm-ar RANLIB=llvm-ranlib \
		CC="$(CC) --target=$(MUSL_TARGET)" \
		CFLAGS="-O3 -fuse-ld=lld" \
		LDFLAGS="-Wl,--build-id=sha1 ../arith.a" \
		LIBCC="" \
		CROSS_COMPILE=$(MUSL_TARGET) \
		--srcdir=../../../../external/musl/src \
		--prefix=/usr \
		--enable-optimize \
		--target=$(MUSL_TARGET)

$(musl-out)override.mk: $(musl-out)config.mak
	$(call action,"GEN  ")
	grep 'ARCH =' $< | sed 's/^/override /g' > $@

$(musl-out)%: $(musl-out)config.mak $(musl-out)override.mk $(musl-out-base)arith.a
	$(call action,"MAKE ")
	rm -f $@
	$(MAKE) -C $(musl-out) -f override.mk -f Makefile

cflags-inc += -isystem $(musl-src)arch/$(MUSL_ARCH)/

default: $(musl-out)lib/libc.so

musl-compiler-rt-builtins := mulxc3.o mulsc3.o muldc3.o udivdi3.o divdi3.o umoddi3.o moddi3.o udivmoddi4.o
define rule_musl_compiler-rt-builtins
$(musl-out-base)arith.a: $(foreach __comprt,$(musl-compiler-rt-builtins),$(compiler-rt-builtins-out)$(__comprt))
endef
make-finalize += rule_musl_compiler-rt-builtins
