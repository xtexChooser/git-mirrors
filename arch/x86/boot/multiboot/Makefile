$(call load-objs, boot.o entry.o)

$(curout)%: NO_PIE = 1

$(curout)multiboot.o: $(curdir)linker.ld $(curout)entry.o $(curout)boot.o $(out)/core/boot/boot.o $(out)/arch/x86/boot/boot.o
	$(ld) -T $< -o $@ $(filter-out $<,$^)
