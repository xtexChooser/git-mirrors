ENTRY(_start)

SECTIONS {
	. = 1M;
	_ELF_BEGIN_ = .;

	.multiboot : {
		KEEP(*(.multiboot))
	}

	.text : ALIGN(4K) {
		*(.text)
	}

	.rodata : ALIGN(4K) {
		*(.rodata)
	}

	.data : ALIGN(4K) {
		*(.data)
	}

	.bss : ALIGN(4K) {
		*(COMMON)
		*(.bss)
	}

	_ELF_END_ = .;
}
