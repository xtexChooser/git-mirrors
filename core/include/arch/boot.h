#include "boot/boot.h"
#ifndef __CORE_ARCH_BOOT_HEADER__
#define __CORE_ARCH_BOOT_HEADER__ 1

/**
 * @brief Archiecture-specific logic after bootloader initialized. Called by arch bootloader
 * 
 */
void arch_boot();

/**
 * @brief Archiecture-specific logic before boot. Called by core/boot
 *
 */
void arch_pre_boot(boot_info_t *bootinfo);

/**
 * @brief Generates a random number for booting.
 * 
 * @return u64 Random number
 */
u64 arch_boot_rand();

#endif
