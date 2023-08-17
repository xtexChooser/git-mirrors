#include <core/mm/mm.hpp>
#include <core/mm/phy/phymm.hpp>
#include <core/utils/log.h>

LOG_TAG("mm");

namespace xos::mm {
void mm_init(boot::boot_info_t *bootinfo) { phy::phymm_init(bootinfo); }
} // namespace xos::mm
