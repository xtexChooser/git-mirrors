subobjs += boot.o entry.o

$(curobj)multiboot.o: $(curdir)linker.ld $(curobj)entry.o $(curobj)boot.o $(obj)/core/boot/boot.o $(obj)/arch/x86/boot/boot.o
	$(LD) $(LDFLAGS) -T $< -o $@ -Wl,-s,--build-id=sha1 $(filter-out $<,$^)
