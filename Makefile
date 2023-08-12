NAME		= xtex-os
VERSION		= 1.0

CC			= clang
LD			= clang
objs		:=
mkparent	= @mkdir -p $$(dirname $@)

OUT			?= out
obj			:= ${OUT}
SRC			?= .
src			:= ${SRC}
VPATH		+= $(src)

cflags		+= -Wall -Werror -Wextra -Wno-error=unused-parameter
cflags		+= -O3 -g
cflags		+= -nostdlib -ffreestanding -fno-exceptions -fno-rtti -std=gnu17 -fno-use-cxa-atexit -fno-builtin
#cflags		+= -fcf-protection # todo: only on x86_64
#cflags		+= -fstack-protector 
cflags		+= -Icore/include -Iarch/$(ARCH)/include
cflags		+= -fPIE
ldflags		+= -Wl,--pie -Wl,--static
ldflags		+= -Wl,--build-id

.DELETE_ON_ERROR:

define load-subdir
$(eval curdir := $1)
$(eval curobj := $(obj)/$1)
$(eval subobjs =)
$(eval subcobjs =)
$(eval subdirs =)
$(eval include $1Makefile)
$(eval subobjs += $$(subcobjs))
$(foreach __subcobj,$(subcobjs),$(eval cobjs += $(obj)/$1$(__subcobj)))
$(foreach __subobj,$(subobjs),$(eval objs += $(obj)/$1$(__subobj)))
$(foreach __subdir,$(subdirs),$(call load-subdir,$1$(__subdir)))
endef

ifeq ($(ARCH),)
$(error ARCH is not specified)
endif
include arch/$(ARCH)/config.mk
include arch/$(ARCH)/base.mk

cflags		+= --target=$(CLANG_TARGET)
ldflags		+= --target=$(CLANG_TARGET)
CFLAGS		+= $(cflags)
LDFLAGS		+= $(cflags) $(ldflags)

export NAME VERSION
export OUT SRC
export CC LD CFLAGS LDFLAGS

default: $(ARCH_DEFAULT_TARGET)
.PHONY: default

subdirs += core/
subdirs += arch/$(ARCH)/

$(foreach __subdir,$(subdirs),$(call load-subdir,$(__subdir)))

include $(patsubst %.o,%.d,$(objs))

$(obj)/%.d: %.c
	$(call mkparent)
	$(CC) $(CFLAGS) -E -M $< -o $@.tmp
	sed 's/^\(.*\):\s/$(obj)\/$(subst /,\/,$(subst .c,.o,$<)): /g' $@.tmp > $@
	rm $@.tmp

$(obj)/%.d: %.S
	$(call mkparent)
	$(CC) $(CFLAGS) -E -M $< -o $@.tmp
	sed 's/^\(.*\):\s/$(obj)\/$(subst /,\/,$(subst .c,.o,$<)): /g' $@.tmp > $@
	rm $@.tmp

$(obj)/%.o: %.c $(obj)/compile_flags.txt
	$(call mkparent)
	$(CC) $(CFLAGS) -c $< -o $@

$(obj)/%.o: %.S $(obj)/compile_flags.txt
	$(call mkparent)
	$(CC) $(CFLAGS) -c $< -o $@

compile_flags_hash=$(shell echo $(CFLAGS) | sha1sum | head -c8)
$(obj)/.compile_flags.$(compile_flags_hash):
	rm -f $(obj)/.compile_flags.*
	echo $(compile_flags_hash) > $@

$(obj)/compile_flags.txt: $(obj)/.compile_flags.$(compile_flags_hash)
	echo $(CFLAGS) | sed 's/\s/\n/g' | sort | uniq > $@

compile_flags.txt: $(obj)/compile_flags.txt
	cp $< $@

clean:
	rm -rf $(obj)/*

.PHONY: clean

$(obj)/core.o: core/linker.ld $(cobjs)
	$(LD) $(LDFLAGS) -T $< -o $@ $(filter-out $<,$^)
