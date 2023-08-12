#ifndef __CORE_BOOT_HEADER__
#define __CORE_BOOT_HEADER__ 1

void boot_reserve_mem(void *addr, void *end);

void do_core_boot();

#endif
