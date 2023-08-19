#ifndef __XOS_ARCH_BOOTLOADER_H__
#define __XOS_ARCH_BOOTLOADER_H__

#include <types.h>

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

/**
 * @brief Allocate a memory block filled with zero
 *
 * @param size Size
 * @return void* Pointer
 */
void *arch_boot_malloc(usize size);

#endif
