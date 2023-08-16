#ifndef __CORE_ARCH_INIT_HEADER__
#define __CORE_ARCH_INIT_HEADER__ 1
#include <boot/boot.h>

/**
 * @brief Do archiecture-specific initializations
 * 
 * @param bootinfo Boot info
 */
void arch_init(boot_info_t *bootinfo);

#endif
