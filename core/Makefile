$(call load-dirs, boot/ utils/)

define rule_core.o
$(eval
$$(out)/core.o: core/linker.ld $$(allobjs-core)
	$$(call action,"LD   ")
	$$(ld) -T $$< -o $$@ $$(filter-out $$<,$$^)
)
endef
make-finalize += rule_core.o
