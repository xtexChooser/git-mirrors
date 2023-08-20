#include "elf.h"
#include <xos/boot/arch.h>
#include <xos/boot/bootloader.h>

void arch_boot() { __asm__("cli"); }

bool arch_pre_boot(boot_info_t *bootinfo) { return true; }

/**
 * @brief Randomize a number. Modified from Wichmann-Hill.
 *
 * Wichmann, B. A., and I. D. Hill. “Algorithm AS 183: An Efficient and Portable
 * Pseudo-Random Number Generator.” Journal of the Royal Statistical Society.
 * Series C (Applied Statistics), vol. 31, no. 2, 1982, pp. 188–90. JSTOR,
 * https://doi.org/10.2307/2347988. Accessed 13 Aug. 2023.
 */
u64 arch_boot_rand_randomize(u64 source) {
	u16 s0 = (u16)source & 0xffff, s1 = (u16)(source >> 16) & 0xffff,
		s2 = (u16)(source >> 32) & 0xffff;
	s0 = (171 * s0) % 30269;
	s1 = (172 * s1) % 30307;
	s2 = (170 * s2) % 30323;
	u32 low = (s0 / 30269.0 + s1 / 30307.0 + s2 / 30323.0) * U32_MAX;

	s0 = (171 * s0) % 30269;
	s1 = (172 * s1) % 30307;
	s2 = (170 * s2) % 30323;
	u32 high = (s0 / 30269.0 + s1 / 30307.0 + s2 / 30323.0) * U32_MAX;
	return ((u64)high) << 32 | low;
}

u64 arch_boot_rand() {
	u32 low, high, cpu_feat;

	asm volatile("cpuid" : "=c"(cpu_feat) : "a"(0x01) : "ebx", "edx");
	if (cpu_feat >> 30 & 1) {
		// RDRAND available
		print("x86/boot: use RDRAND as boot RNG\n");
		asm volatile(".arch_boot_rand_rdrand_retry0:\n\t"
					 "rdrandl %0\n\t"
					 "jnc .arch_boot_rand_rdrand_retry0\n\t"
					 : "=r"(high));
		asm volatile(".arch_boot_rand_rdrand_retry1:\n\t"
					 "rdrandl %0\n\t"
					 "jnc .arch_boot_rand_rdrand_retry1\n\t"
					 : "=r"(low));
		return ((u64)high) << 32 | low;
	}

	print("x86/boot: use RDTSC+WH as boot RNG\n");
	asm volatile("rdtsc" : "=a"(low), "=d"(high));
	return arch_boot_rand_randomize(((u64)high) << 32 | low);
}

bool arch_check_elf32_machine_valid(u16 machine) { return machine == EM_386; }

bool arch_check_elf64_machine_valid(u16 machine) {
	return machine == EM_X86_64 || machine == EM_IA_64;
}

bool arch_do_elf_reloc(arch_boot_reloc_req_t *r) {
	u32 type = r->type;
	if (type == R_386_NONE || type == R_386_COPY) {
		// none
	} else if (type == R_386_32) {
		// S + A
		*(u32 *)r->ptr = reloc_req_symoff(r) + r->addend;
	} else if (type == R_386_PC32 || type == R_386_PC16 || type == R_386_PC8 ||
			   type == R_386_PLT32) {
		// https://git.kernel.org/pub/scm/linux/kernel/git/stable/linux.git/tree/arch/x86/tools/relocs.c?id=1c2f87c22566cd057bc8cde10c37ae9da1a1bb76#n827
		// we dont adjust it
		// S + A - P
		// *(u32 *)r->ptr = reloc_req_symoff(r) + r->addend - r->offset;
	} else if (type == R_386_GOT32 || type == R_386_PLT32 ||
			   type == R_386_GLOB_DAT || type == R_386_JMP_SLOT ||
			   type == R_386_GOTOFF || type == R_386_GOTPC) {
		// ignore it, idk why
	} else if (type == R_386_RELATIVE) {
		// B + A
		*(u32 *)r->ptr = (u32)r->bootinfo->core_load_offset + r->addend;
	} else {
		return false;
	}
	return true;
}
