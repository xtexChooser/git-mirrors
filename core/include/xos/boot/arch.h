#ifndef __XOS_BOOT_ARCH_H__
#define __XOS_BOOT_ARCH_H__

#include <xos/boot/boot.h>
#include <xos/boot/libboot.h>

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

/**
 * @brief A boot-stage ELF relocation request
 *
 */
struct arch_boot_reloc_req {
	/**
	 * @brief Reference to the xos::boot::boot_info struct
	 *
	 */
	boot_info_t *bootinfo;
	/**
	 * @brief The symbol table section
	 * The value of `sh_link` actually
	 *
	 */
	u32 symtab;
	/**
	 * @brief The offset of the position to reloc
	 * The value of `r_offset`
	 *
	 */
	usize offset;
	/**
	 * @brief The pointer to the position to reloc, in memory
	 *
	 */
	void *ptr;
	/**
	 * @brief The value of `r_info` field
	 *
	 */
	u64 info;
	/**
	 * @brief The referenced symbol
	 * Maybe NULL if it is not referenced.
	 * Only lookup this as needed, or else ::lookup_core_symbol may error and
	 * hang up.
	 *
	 */
	u32 sym;
	/**
	 * @brief The relocation type
	 *
	 */
	u32 type;
	/**
	 * @brief The addend for relocation
	 * The value of `r_addend` actually.
	 *
	 */
	u64 addend;
};
typedef struct arch_boot_reloc_req arch_boot_reloc_req_t;

/**
 * @brief Lookup the value of the referenced symbol in a relocation request
 *
 */
#define reloc_req_symoff(r)                                                    \
	lookup_core_symbol((r)->bootinfo, (r)->symtab, (r)->sym)

/**
 * @brief Do a ELF relocation
 *
 * @param r Relocation request
 * @return true Succeeded
 * @return false Failed
 */
bool arch_do_elf_reloc(arch_boot_reloc_req_t *r);

#endif
