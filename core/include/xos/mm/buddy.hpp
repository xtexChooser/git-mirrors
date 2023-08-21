#ifndef __XOS_MM_BUDDY_HPP__
#define __XOS_MM_BUDDY_HPP__

#include <types.h>
#include <xos/arch.hpp>
#include <xos/mm/mm.hpp>

namespace xos::mm::buddy {

namespace impl {
#define BUDDY_ALLOC_ALIGN PAGE_SIZE
#include "external/buddy_alloc/buddy_alloc.h"
} // namespace impl

/**
 * @brief A Buddy memory allocator
 *
 */
class BuddyAllocator : public MemAllocator {
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

	/**
	 * @brief Get the size of a buddy allocator (`sizeof(BuddyAllocator) +
	 * metadata_size`) in bytes
	 *
	 * @param mem_sz Memory size
	 * @return usize Size of the allocator
	 */
	static usize get_size(usize mem_sz);

	void *malloc(usize size) override;
	void free(void *ptr) override;

	void *calloc(usize num, usize size) override;
	void *realloc(void *ptr, usize new_size) override;

	bool reserve(void *ptr, usize size) override;
	bool unreserve(void *ptr, usize size) override;
};
} // namespace xos::mm::buddy

#endif
