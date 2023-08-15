#include "boot/libboot.h"
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

	parse_core_elf(bootinfo);
	if (bootinfo->do_aslr) {
		find_core_boot_mem(bootinfo);
	}
	if (bootinfo->core_load_offset == NULL) {
		if (!check_core_loadable_at(bootinfo, bootinfo->core_load_offset)) {
			print("boot: ASLR disabled or failed, but the core cant be loaded "
				  "at present position\n");
			return;
		}
	}
	load_core(bootinfo);
	if (bootinfo->core_entry == NULL) {
		print("boot: load_core_elf failed to locate the entrypoint\n");
		return;
	}
	if (!arch_pre_boot(bootinfo)) {
		print("boot: arch_pre_boot failed\n");
		return;
	}
	print("boot: calling core_entry\n");
	bootinfo->core_entry = (boot_core_entry *)(bootinfo->core_load_offset +
											   (usize)bootinfo->core_entry);
	char *ret = bootinfo->core_entry(bootinfo);
	print("boot: core entry returned:\n");
	print(ret);
	print("\n");
}

void find_core_boot_mem(boot_info_t *bootinfo) {
	void *load_base = (void *)flooru(bootinfo->random, SZ_4K) + SZ_2M;
	usize core_size = (usize)(bootinfo->core_end - bootinfo->core_start);
	while (1) {
		void *load_end = load_base + core_size;
		if (load_end < (bootinfo->mem_upper - SZ_1M)) {
			if (check_core_loadable_at(bootinfo, load_base)) {
				break;
			}
			load_base = (void *)load_base - SZ_4K;
		} else
			load_base = (void *)flooru((usize)load_base / 2, SZ_4K);
		if ((usize)load_base <= SZ_2M) {
			print("boot: ASLR locate failed\n");
			bootinfo->core_load_offset = NULL;
			return;
		}
	}
	bootinfo->core_load_offset = load_base;
	return;
}

bool check_core_loadable_at(boot_info_t *bootinfo, void *offset) {
	boot_elf_load_t *load = bootinfo->core_elf_load;
	while (load != NULL) {
		void *load_start = offset + load->start;
		void *load_end = load_start + load->size;
		// check bootloader reserve
		if (!check_arch_boot_memory_available(load_start, load_end)) {
			return false;
		}
		// check bootinfo reserve
		boot_reserved_mem_t *memblock = bootinfo->reserved_mem;
		while (memblock != NULL) {
			if (max(memblock->start, load_start) <
				min(memblock->end, load_end)) {
				return false;
			}
			memblock = memblock->next;
		}
		load = load->next;
	}
	return true;
}

void parse_core_elf(boot_info_t *bootinfo) {
	u8(*ident)[EI_NIDENT] = &((Elf32_Ehdr *)bootinfo->core_start)->e_ident;
	switch ((*ident)[EI_CLASS]) {
	case ELFCLASS32:
		parse_core_elf32(bootinfo);
		break;
	case ELFCLASS64:
		parse_core_elf64(bootinfo);
		break;
	default:
		print("boot: unknown EI_CLASS ident in core ELF\n");
		while (1)
			;
	}
}

void parse_core_elf32(boot_info_t *bootinfo) {
	int i;

	Elf32_Ehdr *ehdr = (Elf32_Ehdr *)bootinfo->core_start;
	if (!arch_check_elf32_machine_valid(ehdr->e_machine)) {
		print("boot: invalid e_machine in 32-bits core ELF\n");
		return;
	}
	bootinfo->core_entry = (boot_core_entry *)ehdr->e_entry;
	// parse LOAD phdrs
	boot_elf_load_t **next_core_load = &bootinfo->core_elf_load;
	Elf32_Phdr *phdr =
		(Elf32_Phdr *)(bootinfo->core_start + (usize)ehdr->e_phoff);
	for (i = 0; i < ehdr->e_phnum; i++) {
		if (phdr->p_type == PT_LOAD) {
			boot_elf_load_t *core_load =
				(boot_elf_load_t *)arch_boot_malloc(sizeof(boot_elf_load_t));
			*next_core_load = core_load;
			next_core_load = &core_load->next;
			core_load->offset = (usize)phdr->p_offset;
			core_load->start = (usize)phdr->p_paddr;
			core_load->size = ceilu((usize)phdr->p_memsz, (usize)phdr->p_align);
		}
		phdr = (Elf32_Phdr *)((void *)phdr + (usize)ehdr->e_phentsize);
	}
}

void parse_core_elf64(boot_info_t *bootinfo) {
	int i;

	Elf64_Ehdr *ehdr = (Elf64_Ehdr *)bootinfo->core_start;
	if (!arch_check_elf64_machine_valid(ehdr->e_machine)) {
		print("boot: invalid e_machine in 64-bits core ELF\n");
		return;
	}
	bootinfo->core_entry = (boot_core_entry *)ehdr->e_entry;
	// parse LOAD phdrs
	boot_elf_load_t **next_core_load = &bootinfo->core_elf_load;
	Elf64_Phdr *phdr =
		(Elf64_Phdr *)(bootinfo->core_start + (usize)ehdr->e_phoff);
	for (i = 0; i < ehdr->e_phnum; i++) {
		if (phdr->p_type == PT_LOAD) {
			boot_elf_load_t *core_load =
				(boot_elf_load_t *)arch_boot_malloc(sizeof(boot_elf_load_t));
			*next_core_load = core_load;
			next_core_load = &core_load->next;
			core_load->offset = (usize)phdr->p_offset;
			core_load->start = (usize)phdr->p_paddr;
			core_load->size = ceilu((usize)phdr->p_memsz, (usize)phdr->p_align);
		}
		phdr = (Elf64_Phdr *)((void *)phdr + (usize)ehdr->e_phentsize);
	}
}

void load_core(boot_info_t *bootinfo) {
	boot_elf_load_t *load = bootinfo->core_elf_load;
	while (load != NULL) {
		void *memsrc = bootinfo->core_start + load->offset;
		void *memsrc_end = memsrc + load->size;
		void *memdst = bootinfo->core_load_offset + load->start;
		while (memsrc < memsrc_end && memsrc < bootinfo->core_end) {
			*(u64 *)memdst = *(u64 *)memsrc;
			memsrc += sizeof(u64);
			memdst += sizeof(u64);
		}
		load = load->next;
	}
}
