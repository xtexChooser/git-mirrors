$(call load-objs, boot.o entry.o)

$(curout)%: NO_PIE = 1

$(curout)multiboot.o: $(curdir)linker.ld $(curout)entry.o $(curout)boot.o $(out)/core/boot/libboot.o $(out)/arch/x86/boot/boot.o
	$(call action,"LD   ")
	$(ld) -T $< -o $@ $(filter-out $<,$^)
