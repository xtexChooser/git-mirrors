#ifndef __XOS_MM_PHY_PHYMM_HPP__
#define __XOS_MM_PHY_PHYMM_HPP__

#include <xos/boot/boot.h>
#include <xos/mm/phy/buddy.hpp>

namespace xos::mm::phy {
extern buddy::BuddyAllocator *main_alloc;

void init(boot::boot_info_t *bootinfo);

void *alloc(usize size);
void free(void *ptr);
}

#endif
