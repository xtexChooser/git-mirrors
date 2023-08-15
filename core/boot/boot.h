#include <types.h>
#ifndef __CORE_BOOT_HEADER__
#define __CORE_BOOT_HEADER__ 1

struct boot_reserved_mem {
	struct boot_reserved_mem *next;
	void *start;
	void *end;
};
typedef struct boot_reserved_mem boot_reserved_mem_t;

struct boot_module {
	struct boot_module *next;
	void *start;
	void *end;
};
typedef struct boot_module boot_module_t;

struct boot_elf_load {
	struct boot_elf_load *next;
	usize start;
	usize size;
};
typedef struct boot_elf_load boot_elf_load_t;

typedef struct boot_info boot_info_t;

typedef char *(boot_core_entry)(boot_info_t *bootinfo);

struct boot_info {
	void *mem_upper;
	void *core_start;
	void *core_end;
	boot_reserved_mem_t *reserved_mem;
	boot_module_t *module;
	/**
	 * @brief The random number. Filled by core boot
	 *
	 */
	u64 random;
	/**
	 * @brief If the core DYN. Filled by core boot
	 *
	 */
	bool do_aslr;
	/**
	 * @brief The lower address to load core at. Filled by core boot
	 *
	 */
	void *core_load_offset;
	/**
	 * @brief Information about ELF LOAD program headers. Filled by core boot
	 *
	 */
	boot_elf_load_t *core_elf_load;
	/**
	 * @brief The entrypoint of core. Filled by core boot `load_core_elf`
	 *
	 */
	boot_core_entry *core_entry;
};

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
bool check_core_loadable_at(boot_info_t *bootinfo, void* offset);

/**
 * @brief Copy the core to given address and call load_core_elf32 or
 * load_core_elf64
 *
 * @param bootinfo Boot info
 */
void parse_core_elf(boot_info_t *bootinfo);

void parse_core_elf32(boot_info_t *bootinfo);
void parse_core_elf64(boot_info_t *bootinfo);

/**
 * @brief Load ELF and do relocations
 *
 * @param bootinfo Boot info
 */
void load_core(boot_info_t *bootinfo);

#endif
