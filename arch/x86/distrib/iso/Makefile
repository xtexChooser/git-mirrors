$(curout)boot.iso: $(curout)iso_root/boot/grub/grub.cfg \
	$(curout)iso_root/boot/$(NAME)/multiboot.o \
	$(curout)iso_root/boot/$(NAME)/core.o
	$(call action,"GEN  ")
	if [[ -e "$$(which grub2-mkrescue)" ]]; then
	grub_mkrescue=grub2-mkrescue
	else
	grub_mkrescue=grub-mkrescue
	fi
	$$grub_mkrescue -o $@ --product-name=$(NAME) --product-version=$(VERSION) $(out)/arch/x86/distrib/iso/iso_root

ifneq ($(CONFIG_NO_KASLR),)
GRUB_EXTRA_CMDLINE += nokaslr
endif

$(call saved, $(curout).grub.cfg, x86_iso_grub_cfg_ext_cmdline, GRUB_EXTRA_CMDLINE)

$(curout)iso_root/boot/grub/grub.cfg: $(curdir)grub.cfg $(x86_iso_grub_cfg_ext_cmdline_file)
	$(call action,"GEN  ")
	cat $< | EXTRA_CMDLINE="$(GRUB_EXTRA_CMDLINE)" envsubst > $@

$(curout)iso_root/boot/$(NAME)/multiboot.o: $(out)/arch/x86/boot/multiboot/multiboot.o
	$(call action,"COPY ")
	cp $< $@

$(curout)iso_root/boot/$(NAME)/core.o: $(out)/core/core.o
	$(call action,"COPY ")
	cp $< $@

qemu-iso: $(curout)boot.iso
	$(call action,"RUN  ",$<)
	qemu-system-i386 \
		-drive media=cdrom,file=$<,readonly=on,format=raw \
		-serial stdio \
		-m 256M \
		-cpu Snowridge \
		-s \
		-name $(NAME) \
		$(QEMU_FLAGS)

.PHONY: qemu-iso
