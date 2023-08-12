CC			= clang
LD			= clang
objs		:=
mkparent	= @mkdir -p $$(dirname $@)

OUT			?= out
obj			:= ${OUT}
SRC			?= .
src			:= ${SRC}
VPATH		+= $(src)

cflags		+= -Wall -Werror #-Wextra
cflags		+= -O3 -g
cflags		+= -nostdlib -ffreestanding -fno-exceptions -fno-rtti -std=gnu17 -fno-use-cxa-atexit -fno-builtin
#cflags		+= -fcf-protection # todo: only on x86_64
#cflags		+= -fstack-protector 
cflags		+= -fPIE
ldflags		+= -Wl,--pie -Wl,--static
ldflags		+= -Wl,--build-id

.DELETE_ON_ERROR:

define load-subdir
$(eval curdir := $1)
$(eval curobj := $(obj)/$1)
$(eval subobjs =)
$(eval subdirs =)
$(eval include $1Makefile)
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

$(obj)/%.o: %.c
	$(call mkparent)
	$(CC) $(CFLAGS) -c $< -o $@

$(obj)/%.o: %.S
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
