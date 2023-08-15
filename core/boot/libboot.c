#include "boot/libboot.h"
#include "arch/boot.h"
#include "arch/bootloader.h"
#include "external/musl/src/include/elf.h"
#include "math.h"
#include <limits.h>
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

static void parse_core_elf32(boot_info_t *bootinfo) {
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

static void parse_core_elf64(boot_info_t *bootinfo) {
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
	// Load PT_LOAD
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
	// Do relocations
	u8(*ident)[EI_NIDENT] = &((Elf32_Ehdr *)bootinfo->core_start)->e_ident;
	switch ((*ident)[EI_CLASS]) {
	case ELFCLASS32:
		reloc_core32(bootinfo);
		break;
	case ELFCLASS64:
		reloc_core64(bootinfo);
		break;
	}
}

static void reloc_core32(boot_info_t *bootinfo) {
	int i;
	arch_boot_reloc_req_t req;
	req.bootinfo = bootinfo;
	Elf32_Ehdr *ehdr = (Elf32_Ehdr *)bootinfo->core_start;
	Elf32_Shdr *shdr =
		(Elf32_Shdr *)(bootinfo->core_start + (usize)ehdr->e_shoff);
	for (i = 0; i < ehdr->e_shnum; i++) {
		if (shdr->sh_type == SHT_REL) {
			Elf32_Rel *rel =
				(Elf32_Rel *)(bootinfo->core_start + (usize)shdr->sh_offset);
			void *rel_end = (void *)rel + (usize)shdr->sh_size;
			req.symtab = shdr->sh_link;
			while ((void *)rel < rel_end) {
				req.offset = (usize)rel->r_offset;
				req.ptr = bootinfo->core_load_offset + req.offset;
				req.info = (u64)rel->r_info;
				req.sym = ELF32_R_SYM(req.info);
				req.type = ELF32_R_TYPE(req.info);
				req.addend = 0;
				if (!arch_do_elf_reloc(&req)) {
					print("boot: failed to do an ELF32 REL reloc\n");
				}
				rel ++;
			}
		} else if (shdr->sh_type == SHT_RELA) {
			Elf32_Rela *rel =
				(Elf32_Rela *)(bootinfo->core_start + (usize)shdr->sh_offset);
			void *rel_end = (void *)rel + (usize)shdr->sh_size;
			req.symtab = shdr->sh_link;
			while ((void *)rel < rel_end) {
				req.offset = (usize)rel->r_offset;
				req.ptr = bootinfo->core_load_offset + req.offset;
				req.info = (u64)rel->r_info;
				req.sym = ELF32_R_SYM(req.info);
				req.type = ELF32_R_TYPE(req.info);
				req.addend = (u64)rel->r_addend;
				if (!arch_do_elf_reloc(&req)) {
					print("boot: failed to do an ELF32 RELA reloc\n");
				}
				rel ++;
			}
		}
		shdr = (Elf32_Shdr *)((void *)shdr + (usize)ehdr->e_shentsize);
	}
}

static void reloc_core64(boot_info_t *bootinfo) {
	int i;
	arch_boot_reloc_req_t req;
	req.bootinfo = bootinfo;
	Elf64_Ehdr *ehdr = (Elf64_Ehdr *)bootinfo->core_start;
	Elf64_Shdr *shdr =
		(Elf64_Shdr *)(bootinfo->core_start + (usize)ehdr->e_shoff);
	for (i = 0; i < ehdr->e_shnum; i++) {
		if (shdr->sh_type == SHT_REL) {
			Elf64_Rel *rel =
				(Elf64_Rel *)(bootinfo->core_start + (usize)shdr->sh_offset);
			void *rel_end = (void *)rel + (usize)shdr->sh_size;
			req.symtab = shdr->sh_link;
			while ((void *)rel < rel_end) {
				req.offset = (usize)rel->r_offset;
				req.ptr = bootinfo->core_load_offset + req.offset;
				req.info = (u64)rel->r_info;
				req.sym = ELF64_R_SYM(req.info);
				req.type = ELF64_R_TYPE(req.info);
				req.addend = 0;
				if (!arch_do_elf_reloc(&req)) {
					print("boot: failed to do an ELF64 REL reloc\n");
				}
				rel++;
			}
		} else if (shdr->sh_type == SHT_RELA) {
			Elf64_Rela *rel =
				(Elf64_Rela *)(bootinfo->core_start + (usize)shdr->sh_offset);
			void *rel_end = (void *)rel + (usize)shdr->sh_size;
			req.symtab = shdr->sh_link;
			while ((void *)rel < rel_end) {
				req.offset = (usize)rel->r_offset;
				req.ptr = bootinfo->core_load_offset + req.offset;
				req.info = (u64)rel->r_info;
				req.sym = ELF64_R_SYM(req.info);
				req.type = ELF64_R_TYPE(req.info);
				req.addend = (u64)rel->r_addend;
				if (!arch_do_elf_reloc(&req)) {
					print("boot: failed to do an ELF64 RELA reloc\n");
				}
				rel++;
			}
		}
		shdr = (Elf64_Shdr *)((void *)shdr + (usize)ehdr->e_shentsize);
	}
}

usize lookup_core_symbol(boot_info_t *bootinfo, u32 table, u32 index) {
	u8(*ident)[EI_NIDENT] = &((Elf32_Ehdr *)bootinfo->core_start)->e_ident;
	usize ret;
	switch ((*ident)[EI_CLASS]) {
	case ELFCLASS32:
		ret = lookup_core_symbol32(bootinfo, table, index);
	case ELFCLASS64:
		ret = lookup_core_symbol64(bootinfo, table, index);
	default:
		ret = INT_MAX;
	}
	if (ret == INT_MAX) {
		print("libboot: error in core symbol locating\n");
		while (1)
			;
	}
	return ret;
}

static usize lookup_core_symbol32(boot_info_t *bootinfo, u32 table, u32 index) {
	Elf32_Ehdr *ehdr = (Elf32_Ehdr *)bootinfo->core_start;
	if (table >= ehdr->e_shnum)
		return INT_MAX;
	Elf32_Shdr *shdr =
		&((Elf32_Shdr *)(bootinfo->core_start + (usize)ehdr->e_shoff))[table];
	usize shnum = (usize)shdr->sh_size / (usize)shdr->sh_entsize;
	if (index >= shnum)
		return INT_MAX;
	Elf32_Sym *sym =
		&((Elf32_Sym *)(bootinfo->core_start + shdr->sh_offset))[index];
	if (sym->st_shndx == SHN_UNDEF) {
		if (ELF32_ST_BIND(sym->st_info) & STB_WEAK) {
			return 0;
		} else {
			print("libboot: failed to locate SHN_UNDEF and non-STB_WEAK "
				   "symbols in ELF32\n");
			return INT_MAX;
		}
	} else if (sym->st_shndx == SHN_ABS) {
		return sym->st_value;
	} else {
		Elf32_Shdr *target =
			&((Elf32_Shdr *)(bootinfo->core_start +
							 (usize)ehdr->e_shoff))[sym->st_shndx];
		if (sym->st_shndx >= shnum)
			return INT_MAX;
		return target->sh_addr + sym->st_value;
	}
}

static usize lookup_core_symbol64(boot_info_t *bootinfo, u32 table, u32 index) {
	Elf64_Ehdr *ehdr = (Elf64_Ehdr *)bootinfo->core_start;
	if (table >= ehdr->e_shnum)
		return INT_MAX;
	Elf64_Shdr *shdr =
		&((Elf64_Shdr *)(bootinfo->core_start + (usize)ehdr->e_shoff))[table];
	usize shnum = (usize)shdr->sh_size / (usize)shdr->sh_entsize;
	if (index >= shnum)
		return INT_MAX;
	Elf64_Sym *sym =
		&((Elf64_Sym *)(bootinfo->core_start + shdr->sh_offset))[index];
	if (sym->st_shndx == SHN_UNDEF) {
		if (ELF64_ST_BIND(sym->st_info) & STB_WEAK) {
			return 0;
		} else {
			print("libboot: failed to locate SHN_UNDEF and non-STB_WEAK "
				   "symbols in ELF 64\n");
			return INT_MAX;
		}
	} else if (sym->st_shndx == SHN_ABS) {
		return sym->st_value;
	} else {
		Elf64_Shdr *target =
			&((Elf64_Shdr *)(bootinfo->core_start +
							 (usize)ehdr->e_shoff))[sym->st_shndx];
		if (sym->st_shndx >= shnum)
			return INT_MAX;
		return target->sh_addr + sym->st_value;
	}
}