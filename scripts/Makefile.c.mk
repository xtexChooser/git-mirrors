CC			= clang
LD			= clang
AR			= llvm-ar

cflags		+= -Wall -Werror -Wextra -Wno-error=unused-parameter -Wno-unused-function
cflags		+= -O3 -g
cflags		+= -nostdlib -ffreestanding -fno-exceptions -fno-rtti -fno-use-cxa-atexit -fno-builtin
#cflags		+= -fcf-protection # todo: only on x86_64
#cflags		+= -fstack-protector 
cflags-inc	+= -isystemcore/include -Iarch/$(ARCH)/include -I.
cflags-only += -std=gnu17
cflags-boot	+= -fno-vectorize -fno-tree-vectorize -fno-slp-vectorize
cflags-core	+= -fno-vectorize -fno-tree-vectorize -fno-slp-vectorize # todo: enable AVX for core
cppflags	+= -std=c++20

ldflags		+= -Wl,--static,--build-id=sha1 -fuse-ld=lld

cflags		+= --target=$(CLANG_TARGET)
ldflags		+= --target=$(CLANG_TARGET)

CFLAGS		+= $(cflags) $(cflags-only)
CPPFLAGS	+= $(cflags) $(cppflags)
LDFLAGS		+= $(cflags) $(ldflags)

export CC LD CFLAGS LDFLAGS

define cc
$(CC) $(mk-cflags)
endef
define mk-cflags
$(CFLAGS) $(if $(NO_PIE),-fno-pie,-fPIE -fPIC) $(cflags-inc) $(value cflags-$(OBJ_GROUP)) $(value cflags-$(OBJ_GROUP)-only)
endef

define cc-cpp
$(CC) $(mk-cppflags)
endef
define mk-cppflags
$(CPPFLAGS) $(if $(NO_PIE),-fno-pie,-fPIE -fPIC) $(cflags-inc) $(value cflags-$(OBJ_GROUP)) $(value cppflags-$(OBJ_GROUP))
endef

define ld
$(LD) $(mk-ldflags)
endef
define mk-ldflags
$(LDFLAGS) $(if $(NO_PIE),-fno-pie -Wl$(comma)--no-pie,-fPIE -Wl,-pie) $(value ldflags-$(OBJ_GROUP))
endef

cflags_hash=$(shell echo "$(mk-cflags)" | md5sum | head -c8)
cppflags_hash=$(shell echo "$(mk-cppflags)" | md5sum | head -c8)
ldflags_hash=$(shell echo "$(mk-ldflags)" | md5sum | head -c8)

cflags_hash_file=$(out)/.cflags.$(cflags_hash)
cppflags_hash_file=$(out)/.cppflags.$(cppflags_hash)
ldflags_hash_file=$(out)/.ldflags.$(cflags_hash)

$(cflags_hash_file):
	$(Q)$(mkparent)
	rm -f $(out)/.cflags.*
	touch $@

$(out)/.cflags.txt: $(cflags_hash_file)
	echo "$(mk-cflags)" | sed 's/\s/\n/g' | sort | uniq > $@

$(cppflags_hash_file):
	$(Q)$(mkparent)
	rm -f $(out)/.cppflags.*
	touch $@

$(out)/.cppflags.txt: $(cppflags_hash_file)
	echo "$(mk-cppflags)" | sed 's/\s/\n/g' | sort | uniq > $@

$(ldflags_hash_file):
	$(Q)$(mkparent)
	rm -f $(out)/.ldflags.*
	touch $@

$(out)/.ldflags.txt: $(ldflags_hash_file)
	echo "$(mk-ldflags)" | sed 's/\s/\n/g' | sort | uniq > $@

$(out)/%.d: %.c $(cflags_hash_file)
	$(Q)$(mkparent)
	$(cc) -E -M $< -o $@.tmp
	sed 's/^\(.*\):\s/$(out)\/$(subst /,\/,$(subst .c,.o,$<)): /g' $@.tmp > $@
	rm $@.tmp

$(out)/%.d: %.cpp $(cppflags_hash_file)
	$(Q)$(mkparent)
	$(cc-cpp) -E -M $< -o $@.tmp
	sed 's/^\(.*\):\s/$(out)\/$(subst /,\/,$(subst .cpp,.o,$<)): /g' $@.tmp > $@
	rm $@.tmp

$(out)/%.d: %.S $(cflags_hash_file)
	$(Q)$(mkparent)
	$(cc) -D ASM_FILE -E -M $< -o $@.tmp
	sed 's/^\(.*\):\s/$(out)\/$(subst /,\/,$(subst .S,.o,$<)): /g' $@.tmp > $@
	rm $@.tmp

$(out)/%.o: %.c $(cflags_hash_file)
	$(call action,"CC   ")
	$(cc) -D C_FILE -c $< -o $@

$(out)/%.o: %.S $(cflags_hash_file)
	$(call action,"CC   ")
	$(cc) -D ASM_FILE -c $< -o $@

$(out)/%.o: %.cpp $(cppflags_hash_file)
	$(call action,"CC   ")
	$(cc-cpp) -D CXX_FILE -D CPP_FILE -c $< -o $@

$(curout)multiboot.o: $(curdir)linker.ld $(curout)entry.o $(curout)boot.o $(out)/core/boot/boot.o $(out)/arch/x86/boot/boot.o
	$(ld) -T $< -o $@ $(filter-out $<,$^)

compile_flags.txt: $(out)/.cflags.txt $(out)/.cppflags.txt
# exclude CPP-only and C-only flags
	$(call action,"GEN  ")
	echo "$(cflags) $(cflags-inc)" | sed 's/\s/\n/g' | sort | uniq > $@

compile_commands.json: $(out)/.cflags.txt $(out)/.cppflags.txt
	$(call action,"GEN  ")
	bear --output $@ -- make ARCH=$(ARCH) $(MAKE_FLAGS) --always-make

.PHONY: compile_commands.json

$(out)/%.a:
	$(call action,"AR   ")
	$(AR) rsc $@ $^
