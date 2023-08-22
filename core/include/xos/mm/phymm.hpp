#ifndef __XOS_MM_PHYMM_HPP__
#define __XOS_MM_PHYMM_HPP__

#include <xos/boot/boot.h>
#include <xos/mm/buddy.hpp>

namespace xos::mm::phy {
extern buddy::BuddyAllocator *main_alloc;

void init(boot::boot_info_t *bootinfo);

void *malloc(usize size);
void free(void *ptr);

void reserve(void *ptr, usize size);
void unreserve(void *ptr, usize size);
} // namespace xos::mm::phy

#endif
