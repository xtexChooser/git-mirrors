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
	void *core_load_start;
	/**
	 * @brief The lower address to load core at. Filled by core boot
	 *
	 */
	void *core_load_end;
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

void load_core_elf(boot_info_t *bootinfo);

#endif
