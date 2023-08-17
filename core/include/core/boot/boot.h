#pragma once
#include <types.h>

#ifdef __cplusplus
namespace xos::boot {
extern "C" {
#endif

/**
 * @brief Information about a memory block that is reserved
 *
 */
struct boot_reserved_mem {
	/**
	 * @brief Reference to next
	 *
	 */
	struct boot_reserved_mem *next;
	/**
	 * @brief The lower address
	 *
	 */
	void *start;
	/**
	 * @brief The higher address
	 *
	 */
	void *end;
};
typedef struct boot_reserved_mem boot_reserved_mem_t;

/**
 * @brief Information about a module that is loaded by bootloader
 *
 */
struct boot_module {
	/**
	 * @brief Reference to next
	 *
	 */
	struct boot_module *next;
	/**
	 * @brief The lower address of the data of module
	 *
	 */
	void *start;
	/**
	 * @brief The higher address of the data of module
	 *
	 */
	void *end;
};
typedef struct boot_module boot_module_t;

/**
 * @brief Information about a LOAD program header in the core ELF file
 *
 */
struct boot_elf_load {
	/**
	 * @brief Reference to next
	 *
	 */
	struct boot_elf_load *next;
	/**
	 * @brief The offset of data in the image file
	 *
	 */
	usize offset;
	/**
	 * @brief The offset of data in the memory
	 *
	 */
	usize start;
	/**
	 * @brief The size of data
	 *
	 */
	usize size;
};
typedef struct boot_elf_load boot_elf_load_t;

typedef struct boot_info boot_info_t;

/**
 * @brief The entrypoint of core executable file. Implemented at ::core_init
 *
 */
typedef char *(boot_core_entry)(boot_info_t *bootinfo);

/**
 * @brief Information used by libboot
 *
 */
struct boot_info {
	/**
	 * @brief The highest address of linear-memory
	 *
	 */
	void *mem_upper;
	/**
	 * @brief The lower address of bootloader-provided core image file
	 *
	 */
	void *core_start;
	/**
	 * @brief The higher address of bootloader-provided core image file
	 *
	 */
	void *core_end;
	/**
	 * @brief Machine reserved memory blocks
	 * Note that ::core_start to ::core_end and data memory of
	 * modules are not included in this map.
	 *
	 */
	boot_reserved_mem_t *reserved_mem;
	/**
	 * @brief First-stage modules that is loaded by bootloader
	 *
	 */
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
	 * @brief Information about ELF LOAD program headers. Filled by
	 * ::parse_core_elf
	 *
	 */
	boot_elf_load_t *core_elf_load;
	/**
	 * @brief The entrypoint of core. Filled by ::parse_core_elf
	 *
	 */
	boot_core_entry *core_entry;
};

#ifdef __cplusplus
}
} // namespace xos::boot
#endif
