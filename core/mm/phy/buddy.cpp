#include <xos/math.h>
#include <xos/mm/phy/buddy.hpp>
#include <xos/utils/log.h>
#include <xos/utils/panic.h>

LOG_TAG("mm/buddy");

namespace xos::mm::phy::buddy {

namespace impl {
#define BUDDY_ALLOC_IMPLEMENTATION
/// \todo waiting https://github.com/spaskalev/buddy_alloc/pull/75 to be merged
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wall"
#pragma clang diagnostic ignored "-Wextra"
#pragma clang diagnostic ignored "-Wconstant-logical-operand"
#include "external/buddy_alloc/buddy_alloc.h"
#pragma clang diagnostic pop
} // namespace impl

BuddyAllocator::BuddyAllocator(usize mem_sz, void **metadata_alloc) {
	void *metadata = (void *)*metadata_alloc;
	*metadata_alloc =
		(void *)((usize)*metadata_alloc + impl::buddy_sizeof(mem_sz));
	backend = impl::buddy_init((unsigned char *)metadata,
							   /*nullptr*/ (unsigned char *)PAGE_SIZE,
							   flooru(mem_sz, PAGE_SIZE));
	/// \todo we always skipped the first page because
	/// https://github.com/spaskalev/buddy_alloc/issues/76
	ASSERT_NEQ(backend, nullptr);
}

usize BuddyAllocator::get_size(usize mem_sz) {
	return sizeof(BuddyAllocator) + impl::buddy_sizeof(mem_sz);
}

void *BuddyAllocator::alloc(usize size) {
	return impl::buddy_malloc(backend, size);
}
void BuddyAllocator::free(void *ptr) { impl::buddy_free(backend, ptr); }

void BuddyAllocator::reserve(void *ptr, usize size) {
	impl::buddy_reserve_range(backend, ptr, size);
}

void BuddyAllocator::unreserve(void *ptr, usize size) {
	impl::buddy_unsafe_release_range(backend, ptr, size);
}

} // namespace xos::mm::phy::buddy
