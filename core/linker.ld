ENTRY(_start)
 
SECTIONS
{
	. = 2M;

	.text : ALIGN(4K)
	{
		*(.text)
	}

	.rodata : ALIGN(4K)
	{
		*(.rodata)
	}
	.data : ALIGN(4K)
	{
		*(.data)
	}
    _edata = .;
 
	.bss : ALIGN(4K)
	{
		*(COMMON)
		*(.bss)
	}
    _end = .;

    .note.gnu.build-id :
    {
        *(.note.gnu.build-id)
    }
}
