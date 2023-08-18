$(call load-dirs, boot/ utils/ mm/)

define rule_core.o
$(eval
$$(out)/core/core.o: core/linker.ld $$(allobjs-core)
	$$(call action,"LD   ")
	$$(ld) -T $$< -o $$@ $$(filter-out $$<,$$^)
$$(info $$(allobjs-core))
)
endef
make-finalize += rule_core.o

ifneq ($(CONFIG_NO_KASLR),)
$(curout)%: NO_PIE = 1
endif
