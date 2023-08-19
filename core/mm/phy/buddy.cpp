#include <xos/math.h>
#include <xos/mm/phy/buddy.hpp>
#include <xos/utils/log.h>

LOG_TAG("mm/buddy");

namespace xos::mm::phy::buddy {
const int order_sizes[BUDDY_ALLOC_ORDERS] = BUDDY_ALLOC_ORDER_SIZE;

BuddyAllocator::BuddyAllocator(usize mem_sz, void *bitmap_alloc)
	: mem_size(flooru(mem_sz, BUDDY_ALLOC_MAX_ORDER_SIZE)) {
	int i;
	for (i = 0; i < BUDDY_ALLOC_ORDERS; i++) {
		bitmaps[i] = (Bitmap *)bitmap_alloc;
		bitmap_alloc = (void *)((usize)bitmap_alloc +
								(sizeof(u8) * (mem_size / order_sizes[i] / 8)));
	}
}

usize BuddyAllocator::get_bitmap_size(usize mem_sz) {
	mem_sz = flooru(mem_sz, BUDDY_ALLOC_MAX_ORDER_SIZE);
	usize size = 0;
	int i;
	for (i = 0; i < BUDDY_ALLOC_ORDERS; i++) {
		size += sizeof(u8) * (mem_sz / order_sizes[i] / 8);
	}
	return size;
}
} // namespace xos::mm::phy::buddy
