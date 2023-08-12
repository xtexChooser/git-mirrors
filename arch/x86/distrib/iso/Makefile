$(curobj)boot.iso: $(curobj)iso_root/boot/grub/grub.cfg $(curobj)iso_root/boot/$(NAME)/multiboot.o
	grub2-mkrescue -o $@ --product-name=$(NAME) --product-version=$(VERSION) $(curobj)iso_root

$(curobj)iso_root/boot/grub/grub.cfg: $(curdir)grub.cfg
	$(call mkparent)
	cat $< | envsubst > $@

$(curobj)iso_root/boot/$(NAME)/multiboot.o: $(obj)/arch/x86/boot/multiboot/multiboot.o
	$(call mkparent)
	cp $< $@

qemu-iso: $(curobj)boot.iso
	qemu-system-i386 \
		-drive media=cdrom,file=$<,readonly=on,format=raw \
		-serial stdio \
		-m 256M

.PHONY: qemu-iso
