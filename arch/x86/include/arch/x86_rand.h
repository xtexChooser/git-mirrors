#ifndef __ARCH_X86_RAND_HEADER__
#define __ARCH_X86_RAND_HEADER__ 1

u64 x86rand() {
	u32 low, high;
	asm volatile("rdtsc" : "=a"(low), "=d"(high));
	return ((u64)high) << 32 | low;
}

#endif
