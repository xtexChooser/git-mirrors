default: $(out)/arch/x86/distrib/iso/boot.iso

debug: qemu-iso
debug: QEMU_FLAGS=-S
