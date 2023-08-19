#ifndef __XOS_MM_PHY_BUDDY_HPP__
#define __XOS_MM_PHY_BUDDY_HPP__

/**
 * @brief Count of orders of a buddy allocator
 *
 */
#define BUDDY_ALLOC_ORDERS
#undef BUDDY_ALLOC_ORDERS

#include <xos/arch/mm/buddy.hpp>

namespace xos::mm::phy::buddy {
/**
 * @brief Size of orders of buddy allocators
 *
 */
extern const int order_sizes[BUDDY_ALLOC_ORDERS];

#define BUDDY_ALLOC_MAX_ORDER_SIZE order_sizes[BUDDY_ALLOC_ORDERS - 1]

typedef u8 Bitmap[];

class BuddyAllocator {
private:
	const usize mem_size;

public:
	Bitmap *bitmaps[BUDDY_ALLOC_ORDERS];

	BuddyAllocator(usize mem_sz, void *bitmap_alloc);

	static usize get_bitmap_size(usize mem_sz);

	usize alloc(usize size);
	void free(usize ptr, usize size);
};
} // namespace xos::mm::phy::buddy

#endif
