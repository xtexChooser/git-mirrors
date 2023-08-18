#include "multiboot.h"
#include <core/arch/boot.h>
#include <core/arch/bootloader.h>
#include <core/boot/libboot.h>
#include <core/math.h>
#include <stdarg.h>
#include <types.h>

#define TEXT_VIDEO_BUFFER 0xB8000
#define BOOT_INFO_SIZE 0x2000

extern struct multiboot_header multiboot_header;
static multiboot_info_t *mbi;
static u16 text_x_pos = 0, text_y_pos = 0;
static u32 text_width = 80, text_height = 24;
static bool text_mode;
static boot_info_t *bootinfo;
static void *bootinfo_alloc;

void cmain(u32 magic, multiboot_info_t *mbi);
void clear();
void putchar(char chr);
void *arch_boot_malloc(usize size);

void cmain(u32 magic, multiboot_info_t *info) {
	// check bootloader magic
	if (magic != MULTIBOOT_BOOTLOADER_MAGIC) {
		text_mode = true; // assume EGA text buffer available
		print("multiboot: boot: invalid magic number\n");
		return;
	}

	mbi = info;

	// init EGA text buffer
	text_mode = (mbi->flags & MULTIBOOT_INFO_FRAMEBUFFER_INFO >> 12) &&
				mbi->framebuffer_type == MULTIBOOT_FRAMEBUFFER_TYPE_EGA_TEXT;
	text_width = mbi->framebuffer_width;
	text_height = mbi->framebuffer_height;
	clear();

	// init boot mem allocator
	bootinfo_alloc = (void *)multiboot_header.bss_end_addr;

	// boot
	arch_boot();
	bootinfo = (boot_info_t *)arch_boot_malloc(sizeof(boot_info_t));

	if (mbi->flags & MULTIBOOT_INFO_MEMORY) {
		if (mbi->mem_upper >= (U32_MAX - SZ_1M) / SZ_1K)
			bootinfo->mem_upper = (void *)U32_MAX;
		else
			bootinfo->mem_upper = (void *)(mbi->mem_upper * SZ_1K + SZ_1M);
	} else {
		print("multiboot: boot: MULTIBOOT_INFO_MEMORY not available");
		return;
	}

	if (mbi->flags & MULTIBOOT_INFO_CMDLINE && mbi->cmdline != (u32)NULL) {
		char *cmdline = (char *)mbi->cmdline;
		while (*cmdline != 0)
			cmdline++;
		char *end = cmdline;
		char *dst = (char *)arch_boot_malloc((u32)end - mbi->cmdline + 1);
		bootinfo->cmdline = dst;
		cmdline = (char *)mbi->cmdline;
		while (cmdline <= end) {
			*dst = *cmdline;
			dst++;
			cmdline++;
		}
	} else {
		bootinfo->cmdline = "";
	}

	boot_reserved_mem_t *reserved_mem =
		(boot_reserved_mem_t *)arch_boot_malloc(sizeof(boot_reserved_mem_t));
	bootinfo->reserved_mem = reserved_mem;
	reserved_mem->next = NULL;
	reserved_mem->start = (void *)multiboot_header.load_addr;
	reserved_mem->end = (void *)multiboot_header.bss_end_addr + BOOT_INFO_SIZE;

	reserved_mem =
		(boot_reserved_mem_t *)arch_boot_malloc(sizeof(boot_reserved_mem_t));
	bootinfo->reserved_mem->next = reserved_mem;
	reserved_mem->next = NULL;
	reserved_mem->start = (void *)0;
	reserved_mem->end = (void *)SZ_1M;

	if (mbi->flags & MULTIBOOT_INFO_MEM_MAP) {
		multiboot_memory_map_t *mmap;
		for (mmap = (multiboot_memory_map_t *)mbi->mmap_addr;
			 (void *)mmap < (void *)mbi->mmap_addr + mbi->mmap_length;
			 mmap = (multiboot_memory_map_t *)(mmap + mmap->size +
											   sizeof(mmap->size))) {
			if (mmap->type != MULTIBOOT_MEMORY_AVAILABLE) {
				reserved_mem->next = (boot_reserved_mem_t *)arch_boot_malloc(
					sizeof(boot_reserved_mem_t));
				reserved_mem = reserved_mem->next;
				reserved_mem->start = (void *)mmap->addr;
				reserved_mem->end = (void *)mmap->addr + mmap->len;
			}
		}
	}

	if (mbi->flags & MULTIBOOT_INFO_MODS) {
		if (mbi->mods_count < 1) {
			print("multiboot: boot: at least one module must be provided\n");
			return;
		}
		multiboot_module_t *mod = (multiboot_module_t *)mbi->mods_addr;
		boot_module_t **bootmods = &bootinfo->module;
		u32 i;

		for (i = 0; i < mbi->mods_count; i++, mod++) {
			// there is no need to make memory block for mods reserved.
			// those memory will be reserved by
			// check_arch_boot_memory_available. core will reserve and release
			// them later
			if (i == 0) {
				// core
				bootinfo->core_start = (void *)mod->mod_start;
				bootinfo->core_end = (void *)mod->mod_end;
			} else {
				// module
				boot_module_t *bootmod =
					(boot_module_t *)arch_boot_malloc(sizeof(boot_module_t));
				bootmod->start = (void *)mod->mod_start;
				bootmod->end = (void *)mod->mod_end;
				*bootmods = bootmod;
				bootmods = &bootmod->next;
			}
		}
	} else {
		print("multiboot: boot: multiboot modules not available\n");
		return;
	}

	// reserve memories
	do_core_boot(bootinfo);
}

void clear() {
	if (!text_mode)
		return;
	char *buf = (char *)TEXT_VIDEO_BUFFER;
	char *end_buf = buf + 2 * text_width * text_height;
	while (buf < end_buf) {
		*buf = ' ';
		*(buf + 1) = 7;
		buf += 2;
	}
}

void print(str str) {
	if (!text_mode)
		return;
	while (*str != 0) {
		putchar(*str);
		str++;
	}
}

void putchar(char chr) {
	if (chr == '\n' || chr == '\r') {
		text_x_pos = 0;
		text_y_pos = (text_y_pos + 1) % text_height;
		return;
	}
	char *buf = (char *)(TEXT_VIDEO_BUFFER +
						 2 * (text_x_pos + text_width * text_y_pos));
	*buf = chr;
	*(buf + 1) = 7;
	if (text_x_pos == text_width) {
		text_x_pos = 0;
		text_y_pos = (text_y_pos + 1) % text_height;
	}
	text_x_pos += 1;
}

bool check_arch_boot_memory_available(void *start, void *end) {
	// always reserve memory lower than 1M
	if ((usize)start < SZ_1M) {
		return false;
	}
	// check BIOS memory size info
	if (mbi->flags & MULTIBOOT_INFO_MEMORY &&
		(start < (void *)(mbi->mem_lower * SZ_1K) ||
		 end > (void *)(mbi->mem_upper * SZ_1K + SZ_1M))) {
		return false;
	}
	// check conflict with this bootloader
	if (max((void *)multiboot_header.load_addr, start) <
		min((void *)multiboot_header.bss_end_addr + BOOT_INFO_SIZE, end)) {
		return false;
	}
	// check conflict with mbi itself
	if (max((void *)mbi, start) <
		min((void *)mbi + sizeof(multiboot_info_t), end)) {
		return false;
	}
	// check conflict with mbi
	if (mbi->flags & MULTIBOOT_INFO_MEM_MAP) {
		multiboot_memory_map_t *mmap;
		for (mmap = (multiboot_memory_map_t *)mbi->mmap_addr;
			 (void *)mmap < (void *)mbi->mmap_addr + mbi->mmap_length;
			 mmap = (multiboot_memory_map_t *)(mmap + mmap->size +
											   sizeof(mmap->size))) {
			if (mmap->type != MULTIBOOT_MEMORY_AVAILABLE) {
				// not available
				if (max((void *)mmap->size, start) <
					min((void *)(mmap->size + mmap->len), end)) {
					return false;
				}
			}
		}
	}
	if (mbi->flags & MULTIBOOT_INFO_MODS) {
		multiboot_module_t *mod = (multiboot_module_t *)mbi->mods_addr;
		u32 i;

		for (i = 0; i < mbi->mods_count; i++, mod++) {
			if (max((void *)mod->mod_start, start) <
				min((void *)mod->mod_end, end)) {
				return false;
			}
		}
	}
	return true;
}

void *arch_boot_malloc(usize size) {
	void *ptr = bootinfo_alloc;
	bootinfo_alloc = bootinfo_alloc + size;
	void *memdst = ptr;
	while (memdst < bootinfo_alloc) {
		*(u64 *)memdst = 0;
		memdst += sizeof(u64);
	}
	return ptr;
}
