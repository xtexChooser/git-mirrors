#ifndef __XOS_ARCH_INIT_HPP__
#define __XOS_ARCH_INIT_HPP__

#include <xos/boot/boot.h>
#include <types.h>

namespace xos::init {

/**
 * @brief Do archiecture-specific initializations in the early initialization
 * stage
 *
 * @param bootinfo Boot info
 */
void arch_early_init(boot::boot_info_t *bootinfo);

/**
 * @brief Do archiecture-specific initializations
 *
 * @param bootinfo Boot info
 */
void arch_init(boot::boot_info_t *bootinfo);

} // namespace xos::init

#endif
