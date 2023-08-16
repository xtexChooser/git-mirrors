#ifndef __CORE_BOOT_HEADER__
#define __CORE_BOOT_HEADER__ 1
#include <types.h>

#ifdef __cplusplus
extern "C" {
#endif

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
	usize offset;
	usize start;
	usize size;
};
typedef struct boot_elf_load boot_elf_load_t;

struct boot_elf_dynamic {
	struct boot_elf_load *next;
	usize offset;
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
	 * `parse_core_elf`
	 *
	 */
	boot_elf_load_t *core_elf_load;
	/**
	 * @brief The entrypoint of core. Filled by core boot `parse_core_elf`
	 *
	 */
	boot_core_entry *core_entry;
};

#ifdef __cplusplus
}
#endif

#endif