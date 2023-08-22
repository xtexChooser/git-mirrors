#include <xos/arch/x86/gdt.hpp>
#include <xos/init/arch.hpp>

namespace xos::init {

using namespace arch::x86;

void arch_early_init(boot::boot_info_t *bootinfo) { gdt::init(); }
void arch_init(boot::boot_info_t *bootinfo) {}

} // namespace xos::init
