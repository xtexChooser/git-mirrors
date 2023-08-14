#include "boot/boot.h"
#include "arch/boot.h"
#include "arch/bootloader.h"
#include "external/musl/src/include/elf.h"
#include "math.h"
#include <types.h>

void do_core_boot(boot_info_t *bootinfo) {
	bootinfo->random = arch_boot_rand();

	// check ELF magic
	u8(*ident)[EI_NIDENT] = &((Elf32_Ehdr *)bootinfo->core_start)->e_ident;
	if ((*ident)[EI_MAG0] != ELFMAG0 || (*ident)[EI_MAG1] != ELFMAG1 ||
		(*ident)[EI_MAG2] != ELFMAG2 || (*ident)[EI_MAG3] != ELFMAG3) {
		print("boot: invalid ELF magic in core file\n");
		return;
	}
	bootinfo->do_aslr = ((Elf32_Ehdr *)bootinfo->core_start)->e_type == ET_DYN;
	if (bootinfo->do_aslr)
		print("boot: core is DYN, ASLR enabled\n");
	else
		print("boot: core is not DYN, ASLR disabled\n");

	if (bootinfo->do_aslr) {
		find_core_boot_mem(bootinfo);
	}
	if (bootinfo->core_load_start == NULL) {
		bootinfo->core_load_start = bootinfo->core_start;
		bootinfo->core_load_end = bootinfo->core_end;
		if (!check_arch_boot_memory_available(bootinfo->core_load_start,
											  bootinfo->core_load_end)) {
			print("boot: ASLR disabled or failed, but the \n");
			return;
		}
	}
	load_core_elf(bootinfo);
	if (bootinfo->core_entry == NULL) {
		print("boot: load_core_elf failed to locate the entrypoint\n");
		return;
	}
	if (!arch_pre_boot(bootinfo)) {
		print("boot: arch_pre_boot failed\n");
		return;
	}
	print("boot: calling core_entry\n");
	char *ret = bootinfo->core_entry(bootinfo);
	print(ret);
}

void find_core_boot_mem(boot_info_t *bootinfo) {
	void *load_base = (void *)flooru(bootinfo->random, SZ_4K) + SZ_2M;
	usize core_size = (usize)(bootinfo->core_end - bootinfo->core_start);
	while (1) {
		void *load_end = load_base + core_size;
		if (load_end < (bootinfo->mem_upper - SZ_1M)) {
			if (check_arch_boot_memory_available(load_base, load_end)) {
				boot_reserved_mem_t *memblock = bootinfo->reserved_mem;
				bool passed = true;
				while (memblock != NULL) {
					if (max(memblock->start, load_base) <
						min(memblock->end, load_end)) {
						passed = false;
						break;
					}
					memblock = memblock->next;
				}
				if (passed)
					break;
			}
			load_base = (void *)ceilu((usize)load_base + SZ_4K, SZ_4K);
		} else
			load_base = (void *)flooru((usize)load_base / 2, SZ_4K);
		if ((usize)load_base <= SZ_2M) {
			print("boot: ASLR locate failed\n");
			bootinfo->core_load_start = NULL;
			bootinfo->core_load_end = NULL;
			return;
		}
	}
	bootinfo->core_load_start = load_base;
	bootinfo->core_load_end = (void *)load_base + core_size;
	return;
}

void load_core_elf(boot_info_t *bootinfo) {
	// copy core
	void *memsrc = bootinfo->core_start;
	void *memdst = bootinfo->core_load_start;
	while (memsrc < bootinfo->core_end && memdst < bootinfo->core_load_end) {
		*(u64 *)memdst = *(u64 *)memsrc;
		memsrc += sizeof(u64);
		memdst += sizeof(u64);
	}
	// parse elf
}
