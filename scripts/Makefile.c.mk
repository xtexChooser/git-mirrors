CC			= clang
LD			= clang
AR			= llvm-ar

cflags		+= -O3 -g
cflags		+= -nostdlib -ffreestanding -fno-exceptions -fno-rtti -fno-use-cxa-atexit -fno-builtin
#cflags		+= -fcf-protection # todo: only on x86_64
cflags		+= -fstack-protector
cflags-inc	+= -isystem arch/$(ARCH)/include -isystemcore/include -I.
cflags-only += -std=gnu17 -fPIC -nostdinc
cflags-boot	+= -fno-vectorize -fno-tree-vectorize -fno-slp-vectorize
cflags-core	+= -fno-vectorize -fno-tree-vectorize -fno-slp-vectorize # todo: enable AVX for core
cflags-error+= -Wall -Werror -Wextra -Wno-unused-parameter -Wno-unused-function
cppflags	+= -std=c++20 -fPIC -fsized-deallocation -nostdinc -nostdlib++ -nostdinc++

ldflags		+= -Wl,--static,--build-id=sha1 -fuse-ld=lld

cflags		+= --target=$(CLANG_TARGET)
ldflags		+= --target=$(CLANG_TARGET)

CFLAGS		= $(cflags) $(cflags-only)
CPPFLAGS	= $(cflags) $(cppflags)
LDFLAGS		= $(cflags) $(ldflags)

cflags		+= $(if $(CONFIG_DEBUG),-g)
ldflags		+= $(if $(CONFIG_NORELRO),-z norelro,-z now -z relro)
ldflags		+= $(if $(CONFIG_EXECSTACK),-z execstack,-z noexecstack)
ldflags		+= $(if $(CONFIG_NOCOMBRELOC),-z nocombreloc,-z combreloc)

export CC LD CFLAGS LDFLAGS

define mk-cflags-error
$(if $(NO_CFLAGS_ERROR),,$(cflags-error))
endef

define cc
$(CC) $(mk-cflags)
endef
define mk-cflags
$(CFLAGS) $(if $(NO_PIE),-fno-pie,-fPIE) $(mk-cflags-error) $(cflags-inc) $(cflags-$(OBJ_GROUP)) $(cflags-$(OBJ_GROUP)-only)
endef

define cc-cpp
$(CC) $(mk-cppflags)
endef
define mk-cppflags
$(CPPFLAGS) $(if $(NO_PIE),-fno-pie,-fPIE) $(mk-cflags-error) $(cflags-inc) $(cflags-$(OBJ_GROUP)) $(cppflags-$(OBJ_GROUP))
endef

define ld
$(LD) $(mk-ldflags)
endef
define mk-ldflags
$(LDFLAGS) $(if $(NO_PIE),-fno-pie -Wl$(comma)--no-pie,-fPIE -Wl,-pie) $(call mk-cflags-error) $(value ldflags-$(OBJ_GROUP))
endef

$(call saved, $(out)/.cflags, cflags, mk-cflags)
$(call saved, $(out)/.cppflags, cppflags, mk-cppflags)
$(call saved, $(out)/.ldflags, ldflags, mk-ldflags)

$(out)/%.d: %.c $(cflags_file)
	$(Q)$(mkparent)
	$(cc) -E -M $< -o $@.tmp
	sed 's/^\(.*\):\s/$(out)\/$(subst /,\/,$(subst .c,.o,$<)): /g' $@.tmp > $@
	rm $@.tmp

$(out)/%.d: %.cpp $(cppflags_file)
	$(Q)$(mkparent)
	$(cc-cpp) -E -M $< -o $@.tmp
	sed 's/^\(.*\):\s/$(out)\/$(subst /,\/,$(subst .cpp,.o,$<)): /g' $@.tmp > $@
	rm $@.tmp

$(out)/%.d: %.S $(cflags_file)
	$(Q)$(mkparent)
	$(cc) -D ASM_FILE -E -M $< -o $@.tmp
	sed 's/^\(.*\):\s/$(out)\/$(subst /,\/,$(subst .S,.o,$<)): /g' $@.tmp > $@
	rm $@.tmp

$(out)/%.o: %.c $(cflags_file)
	$(call action,"CC   ")
	$(cc) -D C_FILE -c $< -o $@

$(out)/%.o: %.S $(cflags_file)
	$(call action,"CC   ")
	$(cc) -D ASM_FILE -c $< -o $@

$(out)/%.o: %.cpp $(cppflags_file)
	$(call action,"CC   ")
	$(cc-cpp) -D CXX_FILE -D CPP_FILE -c $< -o $@

compile_flags.txt: $(cflags_file)
# exclude CPP-only and C-only flags
	$(call action,"GEN  ")
	echo "$(cflags) $(cflags-inc)" | sed 's/\s/\n/g' | sort | uniq > $@

compile_commands.json: $(cflags_file) $(cppflags_file) $(ldflags_file)
	$(call action,"GEN  ")
	bear --output $@ -- make ARCH=$(ARCH) $(MAKE_FLAGS) --always-make

.PHONY: compile_commands.json

$(out)/%.a:
	$(call action,"AR   ")
	$(AR) rsc $@ $^

define finalize_c_load_depfiles
$(eval include $$(patsubst %.o,%.d,$$(allobjs)))
endef
make-finalize += $(if $(CONFIG_CI),,finalize_c_load_depfiles)
