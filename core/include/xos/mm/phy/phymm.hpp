#ifndef __XOS_MM_PHY_PHYMM_HPP__
#define __XOS_MM_PHY_PHYMM_HPP__

#include <xos/boot/boot.h>

namespace xos::mm::phy {
void phymm_init(boot::boot_info_t *bootinfo);
}

#endif
