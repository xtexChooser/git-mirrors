#ifndef __XOS_MM_PHY_BUDDY_HPP__
#define __XOS_MM_PHY_BUDDY_HPP__

#include <types.h>
#include <xos/arch.hpp>

namespace xos::mm::phy::buddy {

namespace impl {
#define BUDDY_ALLOC_ALIGN PAGE_SIZE
#include "external/buddy_alloc/buddy_alloc.h"
} // namespace impl

/**
 * @brief A Buddy memory allocator
 *
 */
class BuddyAllocator {
private:
	struct impl::buddy *backend;

public:
	/**
	 * @brief Construct a new Buddy Allocator object
	 *
	 * @param mem_sz Memory size
	 * @param metadata_alloc Metadata allocator
	 */
	BuddyAllocator(usize mem_sz, void **metadata_alloc);

	static usize get_size(usize mem_sz);

	void *alloc(usize size);
	void free(void *ptr);

	void reserve(void *ptr, usize size);
	void unreserve(void *ptr, usize size);
};
} // namespace xos::mm::phy::buddy

#endif
