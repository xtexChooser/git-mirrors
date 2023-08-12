ENTRY(_start)
OUTPUT_FORMAT(elf32-i386)
 
SECTIONS
{
	. = 1M;

    .multiboot :
    {
        *(.multiboot)
    }

	.text BLOCK(4K) : ALIGN(4K)
	{
		*(.text)
	}
 
	/* First put the multiboot header, as it is required to be put very early
	   early in the image or the bootloader won't recognize the file format.
	   Next we'll put the .text section. */
	.text BLOCK(4K) : ALIGN(4K)
	{
		*(.text)
	}
 
	/* Read-only data. */
	.rodata BLOCK(4K) : ALIGN(4K)
	{
		*(.rodata)
	}
 
	/* Read-write data (initialized) */
	.data BLOCK(4K) : ALIGN(4K)
	{
		*(.data)
	}

    _edata = .;
 
	/* Read-write data (uninitialized) and stack */
	.bss BLOCK(4K) : ALIGN(4K)
	{
		*(COMMON)
		*(.bss)
	}
    _end = .;

    .note.gnu.build-id :
    {
        *(.note.gnu.build-id)
    }
 
	/* The compiler may produce other sections, by default it will put them in
	   a segment with the same name. Simply add stuff here as needed. */
}
