#ifndef __CORE_ARCH_BOOT_HEADER__
#define __CORE_ARCH_BOOT_HEADER__ 1

/**
 * @brief Archiecture-specific logic after bootloader initialized.
 * 
 */
void arch_boot();

/**
 * @brief Archiecture-specific logic before boot.
 *
 */
void arch_pre_boot();

#endif
