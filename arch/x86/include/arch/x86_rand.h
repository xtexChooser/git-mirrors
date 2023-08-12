#ifndef __ARCH_X86_RAND_HEADER__
#define __ARCH_X86_RAND_HEADER__ 1

#include <immintrin.h>

u64 x86rand() {
	u32 low, high;
	_rdrand32_step(&low);
	_rdrand32_step(&high);
	return ((u64)high) << 32 | low;
}

#endif
