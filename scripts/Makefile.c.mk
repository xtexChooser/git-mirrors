CC			= clang
LD			= clang

cflags		+= -Wall -Werror -Wextra -Wno-error=unused-parameter -Wno-unused-function
cflags		+= -O3 -g
cflags		+= -nostdlib -ffreestanding -fno-exceptions -fno-rtti -std=gnu17 -fno-use-cxa-atexit -fno-builtin
#cflags		+= -fcf-protection # todo: only on x86_64
#cflags		+= -fstack-protector 
cincludes	+= -isystemcore -Iarch/$(ARCH)/include -includecore/types.h -I.

ldflags		+= -Wl,--static,--build-id=sha1 -fuse-ld=lld

cflags		+= --target=$(CLANG_TARGET)
ldflags		+= --target=$(CLANG_TARGET)

CFLAGS		+= $(cflags)
LDFLAGS		+= $(cflags) $(ldflags)

export CC LD CFLAGS LDFLAGS

define cc
$(CC) $(mk-cflags)
endef
define mk-cflags
$(CFLAGS) $(if $(NO_PIE),,-fPIE -fPIC) $(cincludes)
endef

define ld
$(LD) $(mk-ldflags)
endef
define mk-ldflags
$(LDFLAGS) $(if $(NO_PIE),,-fPIE -Wl,-pie)
endef

cflags_hash=$(shell echo "$(mk-cflags)" | md5sum | head -c8)
ldflags_hash=$(shell echo "$(mk-ldflags)" | md5sum | head -c8)

cflags_hash_file=$(out)/.cflags.$(cflags_hash)
ldflags_hash_file=$(out)/.ldflags.$(cflags_hash)

$(cflags_hash_file):
	rm -f $(out)/.cflags.*
	touch $@

$(out)/.cflags.txt: $(cflags_hash_file)
	echo "$(mk-cflags)" | sed 's/\s/\n/g' | sort | uniq > $@

$(ldflags_hash_file):
	rm -f $(out)/.ldflags.*
	touch $@

$(out)/.ldflags.txt: $(ldflags_hash_file)
	echo "$(mk-ldflags)" | sed 's/\s/\n/g' | sort | uniq > $@

compile_flags.txt: $(out)/.cflags.txt
	cp $< $@

default: compile_flags.txt

$(out)/%.d: %.c $(cflags_hash_file)
	$(Q)$(mkparent)
	$(cc) -E -M $< -o $@.tmp
	sed 's/^\(.*\):\s/$(out)\/$(subst /,\/,$(subst .c,.o,$<)): /g' $@.tmp > $@
	rm $@.tmp

$(out)/%.d: %.S $(cflags_hash_file)
	$(Q)$(mkparent)
	$(cc) -D ASM_FILE -E -M $< -o $@.tmp
	sed 's/^\(.*\):\s/$(out)\/$(subst /,\/,$(subst .S,.o,$<)): /g' $@.tmp > $@
	rm $@.tmp

$(out)/%.o: %.c $(cflags_hash_file)
	$(call action,"CC   ")
	$(cc) -c $< -o $@

$(out)/%.o: %.S $(cflags_hash_file)
	$(call action,"CC   ")
	$(cc) -D ASM_FILE -c $< -o $@

$(curout)multiboot.o: $(curdir)linker.ld $(curout)entry.o $(curout)boot.o $(out)/core/boot/boot.o $(out)/arch/x86/boot/boot.o
	$(ld) -T $< -o $@ $(filter-out $<,$^)