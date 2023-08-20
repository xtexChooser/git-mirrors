#ifndef __XOS_MM_PHY_PHYMM_HPP__
#define __XOS_MM_PHY_PHYMM_HPP__

#include <xos/boot/boot.h>
#include <xos/mm/phy/buddy.hpp>

namespace xos::mm::phy {
extern buddy::BuddyAllocator *main_alloc;

void phymm_init(boot::boot_info_t *bootinfo);
}

#endif
