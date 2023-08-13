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

struct boot_info {
	void *mem_upper;
	/**
	 * @brief The random number. Filled by `do_core_boot`
	 * 
	 */
	u64 random;
	void *core_start;
	void *core_end;
	boot_reserved_mem_t *reserved_mem;
	boot_module_t *module;
};
typedef struct boot_info boot_info_t;

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
 * @return void* The lower address of the block
 */
void *find_core_boot_mem(boot_info_t *bootinfo);

#endif
