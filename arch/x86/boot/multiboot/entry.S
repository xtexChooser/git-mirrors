#include "multiboot.h"

#ifdef HAVE_ASM_USCORE
# define C_FUNC(sym) _ ## sym
#else
# define C_FUNC(sym) sym
#endif

#define STACK_SIZE	0x1000

#ifdef __ELF__
# define AOUT_KLUDGE 0
#else
# define AOUT_KLUDGE MULTIBOOT_AOUT_KLUDGE
#endif

#define MULTIBOOT_HEADER_FLAGS	MULTIBOOT_PAGE_ALIGN | MULTIBOOT_MEMORY_INFO | MULTIBOOT_VIDEO_MODE | AOUT_KLUDGE

	.section .multiboot, "ax", @progbits
	.align  4
	.globl multiboot_header

multiboot_header:
	.long   MULTIBOOT_HEADER_MAGIC // magic
	.long   MULTIBOOT_HEADER_FLAGS // flags
	.long   -(MULTIBOOT_HEADER_MAGIC + MULTIBOOT_HEADER_FLAGS) // checksum
	.long   multiboot_header // header_addr
	.long   _start // load_addr
	.long   _edata // load_end_addr
	.long   _end // bss_end_addr
	.long   multiboot_entry // entry_addr
	.long 1 // mode_type
	.long 1024 // width
	.long 600 // height
	.long 32 // depth

	.text
	.globl  _start, multiboot_entry

_start:
multiboot_entry:
	movl $(stack + STACK_SIZE), %esp

	// reset eflags
	pushl $0
	popf

	pushl %ebx
	pushl %eax
	call C_FUNC(cmain)

    pushl $halt_message
	call C_FUNC(print)
	
loop:
	hlt
	jmp loop

halt_message:	.asciz "multiboot: entry: cmain() returned\nmultiboot: entry: halted"

.bss
.comm stack, STACK_SIZE
