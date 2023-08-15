#ifndef __CORE_ARCH_BOOT_HEADER__
#define __CORE_ARCH_BOOT_HEADER__ 1
#include "boot/boot.h"
#include "boot/libboot.h"

/**
 * @brief Archiecture-specific logic after bootloader initialized. Called by
 * arch bootloader
 *
 */
void arch_boot();

/**
 * @brief Archiecture-specific logic before boot. Called by core/boot
 *
 */
bool arch_pre_boot(boot_info_t *bootinfo);

/**
 * @brief Generates a random number for booting.
 *
 * @return u64 Random number
 */
u64 arch_boot_rand();

/**
 * @brief Check if a e_machine value in core 32bits ELF is valid
 *
 * @param machine e_machine
 * @return true Valid
 * @return false Invalid
 */
bool arch_check_elf32_machine_valid(u16 machine);

/**
 * @brief Check if a e_machine value in core 64bits ELF is valid
 *
 * @param machine e_machine
 * @return true Valid
 * @return false Invalid
 */
bool arch_check_elf64_machine_valid(u16 machine);

struct arch_boot_reloc_req {
	boot_info_t *bootinfo;
	u32 symtab;
	usize offset;
	void *ptr;
	u64 info;
	u32 sym;
	u32 type;
	u64 addend;
};
typedef struct arch_boot_reloc_req arch_boot_reloc_req_t;

#define reloc_req_symoff(r)                                                    \
	lookup_core_symbol((r)->bootinfo, (r)->symtab, (r)->sym)

bool arch_do_elf_reloc(arch_boot_reloc_req_t *r);

#endif
