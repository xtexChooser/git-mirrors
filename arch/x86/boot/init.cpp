#include <xos/init/arch.hpp>

namespace xos::init {

void arch_early_init(boot::boot_info_t *bootinfo) { _unused(bootinfo); }
void arch_init(boot::boot_info_t *bootinfo) { _unused(bootinfo); }

} // namespace xos::init
