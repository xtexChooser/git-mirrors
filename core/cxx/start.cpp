#include "cxx.hpp"
#include <elf.h>
#include <xos/boot/boot.h>

extern "C" {

const char *core_init(xos::boot::boot_info_t *bootinfo);
void _call_init_array(void *elf, usize offset);

const char *_start(xos::boot::boot_info_t *bootinfo) {
	_init();
	_call_init_array(bootinfo->core_start, (usize)bootinfo->core_load_offset);
	__stack_chk_init(bootinfo->random);
	const char *ret = core_init(bootinfo);
	_fini();
	__cxa_finalize(nullptr);
	return ret;
}

void _call_init_array(void *elf, usize offset) {
	u8(*ident)[EI_NIDENT] = &((Elf32_Ehdr *)elf)->e_ident;
	if ((*ident)[EI_CLASS] == ELFCLASS32) {
		Elf32_Ehdr *ehdr = (Elf32_Ehdr *)elf;
		Elf32_Shdr *shdr = (Elf32_Shdr *)((usize)elf + ehdr->e_shoff);
		for (int i = 0; i < ehdr->e_shnum; i++) {
			if (shdr->sh_type == SHT_INIT_ARRAY) {
				usize array = (usize)elf + shdr->sh_offset;
				int n = shdr->sh_size;
				for (int p = 0; p < n; p += sizeof(u32))
					((void (*)(void))(offset + *(u32 *)(array + p)))();
			}
			shdr = (Elf32_Shdr *)((usize)shdr + ehdr->e_shentsize);
		}
	} else if ((*ident)[EI_CLASS] == ELFCLASS64) {
		Elf64_Ehdr *ehdr = (Elf64_Ehdr *)elf;
		Elf64_Shdr *shdr = (Elf64_Shdr *)((usize)elf + ehdr->e_shoff);
		for (int i = 0; i < ehdr->e_shnum; i++) {
			if (shdr->sh_type == SHT_INIT_ARRAY) {
				usize array = (usize)elf + shdr->sh_offset;
				int n = shdr->sh_size;
				for (int p = 0; p < n; p += sizeof(u64))
					((void (*)(void))(offset + *(u64 *)(array + p)))();
			}
			shdr = (Elf64_Shdr *)((usize)shdr + ehdr->e_shentsize);
		}
	}
}
}
