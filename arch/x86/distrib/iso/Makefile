$(curout)boot.iso: $(curout)iso_root/boot/grub/grub.cfg \
	$(curout)iso_root/boot/$(NAME)/multiboot.o \
	$(curout)iso_root/boot/$(NAME)/core.o
	grub2-mkrescue -o $@ --product-name=$(NAME) --product-version=$(VERSION) $(out)/arch/x86/distrib/iso/iso_root

$(curout)iso_root/boot/grub/grub.cfg: $(curdir)grub.cfg
	$(call action,"GEN  ")
	cat $< | envsubst > $@

$(curout)iso_root/boot/$(NAME)/multiboot.o: $(out)/arch/x86/boot/multiboot/multiboot.o
	$(call action,"GEN  ")
	cp $< $@

$(curout)iso_root/boot/$(NAME)/core.o: $(out)/core.o
	$(call action,"GEN  ")
	cp $< $@

qemu-iso: $(curout)boot.iso
	$(call action,"RUN  ",$<)
	qemu-system-i386 \
		-drive media=cdrom,file=$<,readonly=on,format=raw \
		-serial stdio \
		-m 256M \
		-cpu Snowridge

.PHONY: qemu-iso
