#include "boot/boot.h"
#include "arch/boot.h"
#include "arch/bootloader.h"
#include "math.h"
#include <types.h>

static void itoa(char *buf, int base, int d) {
	char *p = buf;
	char *p1, *p2;
	unsigned long ud = d;
	int divisor = 10;

	/* If %d is specified and D is minus, put ‘-’ in the head. */
	if (base == 'd' && d < 0) {
		*p++ = '-';
		buf++;
		ud = -d;
	} else if (base == 'x')
		divisor = 16;

	/* Divide UD by DIVISOR until UD == 0. */
	do {
		int remainder = ud % divisor;

		*p++ = (remainder < 10) ? remainder + '0' : remainder + 'a' - 10;
	} while (ud /= divisor);

	/* Terminate BUF. */
	*p = 0;

	/* Reverse BUF. */
	p1 = buf;
	p2 = p - 1;
	while (p1 < p2) {
		char tmp = *p1;
		*p1 = *p2;
		*p2 = tmp;
		p1++;
		p2--;
	}
}

void do_core_boot(boot_info_t *bootinfo) {
	// find core load base
	// check_arch_boot_memory_available
	bootinfo->random = arch_boot_rand();
	char buf[20];
	void *core_start = find_core_boot_mem(bootinfo);
	itoa(buf, 'x', (unsigned)core_start);
	print(buf);
	arch_pre_boot(bootinfo);
}

void *find_core_boot_mem(boot_info_t *bootinfo) {
	// Find with ASLR
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
			return NULL;
		}
	}
	return load_base;
}
