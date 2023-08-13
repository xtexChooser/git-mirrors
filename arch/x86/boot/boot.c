#include "arch/boot.h"
#include <arch/bootloader.h>

void arch_boot() { __asm__("cli"); }

bool arch_pre_boot(boot_info_t *bootinfo) {
	_unused(bootinfo);
	return true;
}

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
