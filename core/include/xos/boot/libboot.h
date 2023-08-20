#ifndef __XOS_BOOT_LIBBOOT_H__
#define __XOS_BOOT_LIBBOOT_H__

#include <types.h>
#include <xos/boot/boot.h>

/**
 * @brief Boot core with given info
 *
 * @param bootinfo Boot info
 */
void do_core_boot(boot_info_t *bootinfo);

/**
 * @brief Find a memory block that can be used to load the kernel
 *
 * @param bootinfo Boot info
 */
void find_core_boot_mem(boot_info_t *bootinfo);

/**
 * @brief Check if the core can be loaded with offset without conflicting with
 * reserved memories
 *
 * @param bootinfo Boot info
 * @param offset Offset to load core at
 * @return true Loadable
 * @return false Not loadable
 */
bool check_core_loadable_at(boot_info_t *bootinfo, void *offset);

/**
 * @brief Copy the core to given address and call load_core_elf32 or
 * load_core_elf64
 *
 * @param bootinfo Boot info
 */
void parse_core_elf(boot_info_t *bootinfo);

static void parse_core_elf32(boot_info_t *bootinfo);
static void parse_core_elf64(boot_info_t *bootinfo);

/**
 * @brief Add core LOAD memory blocks to xos::boot::boot_info::reserved_mem
 *
 * @param bootinfo Boot info
 */
void reserve_core_mem(boot_info_t *bootinfo);

/**
 * @brief Load ELF and do relocations
 *
 * @param bootinfo Boot info
 */
void load_core(boot_info_t *bootinfo);

static void reloc_core32(boot_info_t *bootinfo);
static void reloc_core64(boot_info_t *bootinfo);

/**
 * @brief Try to lookup a symbol in the core SYMTAB
 *
 * @param bootinfo Boot info
 * @param table Symbol table section
 * @param index Symbol index
 * @return usize Pointer
 */
usize lookup_core_symbol(boot_info_t *bootinfo, u32 table, u32 index);

static usize lookup_core_symbol32(boot_info_t *bootinfo, u32 table, u32 index);
static usize lookup_core_symbol64(boot_info_t *bootinfo, u32 table, u32 index);

#endif
