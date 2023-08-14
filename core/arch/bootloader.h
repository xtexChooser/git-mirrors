#ifndef __CORE_ARCH_BOOTLOADER_HEADER__
#define __CORE_ARCH_BOOTLOADER_HEADER__ 1

void print(str str);

/**
 * @brief Check if a memory block is free-to-use for core.
 *
 * @param start The lower address
 * @param end The higher address
 * @return true Available
 * @return false NotAvailable
 */
bool check_arch_boot_memory_available(void *start, void *end);

#endif
