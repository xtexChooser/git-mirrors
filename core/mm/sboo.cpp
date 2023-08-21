#include <algorithm>
#include <cstring>
#include <types.h>
#include <xos/arch.hpp>
#include <xos/math.h>
#include <xos/mm/sboo.hpp>
#include <xos/utils/log.h>
#include <xos/utils/panic.h>

LOG_TAG("mm/sboo");

using namespace std;

namespace xos::mm::sboo {

SbooAllocator::SbooAllocator(MemAllocator *arena_alloc,
							 MemAllocator *bitmap_alloc, u32 object_size)
	: arena_alloc(arena_alloc), bitmap_alloc(bitmap_alloc),
	  objsize(object_size), bitmap_size(max(PAGE_SIZE / object_size / 8, 1u)) {
	if (objsize > 64) {
		// put pointer to pool object before the first object
		first_bitmap = 1; // for the first object
		usize bits = std::max(ceilu(sizeof(sboo_pool), objsize), 1u);
		first_object_offset = bits * objsize;
		for (; bits > 0; bits--) {
			first_bitmap <<= 1;
			first_bitmap |= 1;
		}
	} else
		first_bitmap = 0;
}

SbooAllocator::~SbooAllocator() {}

/// @todo: spin lock
void *SbooAllocator::malloc(u32 size) {
	if (size > objsize)
		return nullptr;
	sboo_pool_t *pool = partial;
	if (pool != nullptr) {
		// find in the first partial pool
		ASSERT_FALSE(pool->full);
		usize bitmap = (usize)pool + sizeof(sboo_pool);
		usize offset = 0;
		if (bitmap_size > sizeof(u64))
			while (*(u64 *)(bitmap + offset) == U64_MAX)
				offset += sizeof(u64);
		if (bitmap_size > sizeof(u32))
			while (*(u32 *)(bitmap + offset) == U32_MAX)
				offset += sizeof(u32);
		/*if (bitmap_size > sizeof(u16))
			while (*(u16 *)(bitmap + offset) == U16_MAX)
				offset += sizeof(u16);*/
		if (bitmap_size > sizeof(u8))
			while (*(u8 *)(bitmap + offset) == U8_MAX)
				offset += sizeof(u8);
		ASSERT_TRUE(offset < bitmap_size);
		u8 btm = *(u8 *)(bitmap + offset);
		u8 bit = 0;
		while (((btm >> bit) & 1) == 1)
			bit++;
		ASSERT_TRUE(bit < 8);
		// mark as used
		btm |= (1 << bit);
		*(u8 *)(bitmap + offset) = btm;
		// check full
		if (btm == U8_MAX) {
			if (offset == bitmap_size - 1)
				pool->full = true;
			else {
				usize offset1 = offset;
				if (bitmap_size > sizeof(u64))
					while (*(u64 *)(bitmap + offset1) == U64_MAX &&
						   offset1 < bitmap_size)
						offset1 += sizeof(u64);
				while (*(u8 *)(bitmap + offset1) == U8_MAX &&
					   offset1 < bitmap_size)
					offset1 += sizeof(u8);
				if (offset1 >= bitmap_size)
					pool->full = true;
			}
			if (pool->full) {
				// pool is full
				// we move the pool to full list
				partial = pool->next;
				pool->next = full;
				full = pool;
			}
		}
		return reinterpret_cast<void *>((usize)pool->page +
										objsize * (offset * 8 + bit));
	} else {
		// alloc new page
		partial = pool = reinterpret_cast<sboo_pool_t *>(
			bitmap_alloc->malloc(sizeof(sboo_pool_t) + bitmap_size));
		if (pool == nullptr)
			return nullptr;
		void *page = arena_alloc->malloc(PAGE_SIZE);
		if (page == nullptr)
			return nullptr;
		pool->full = false;
		pool->next = nullptr;
		pool->page = page;
		usize bitmap = (usize)pool + sizeof(sboo_pool);
		memset((void *)bitmap, 0, bitmap_size);
		// memset((void *)page, 0, PAGE_SIZE);
		// place the pointer to the pool object and allocate the first object
		if (first_bitmap != 0) {
			*(u8 *)bitmap = first_bitmap;
			*(sboo_pool_t **)page = pool;
			return reinterpret_cast<void *>((usize)page + first_object_offset);
		} else {
			*(u8 *)bitmap = 1;
			return page;
		}
	}
}

void SbooAllocator::free(void *ptr) {
	/*sboo_pool_t *pool;
	if (first_bitmap != 0) {
		// fast pointer available

	}*/
}

void *SbooAllocator::realloc(void *ptr, usize new_size) { return nullptr; }

bool SbooAllocator::reserve(void *ptr, usize size) { return false; }

bool SbooAllocator::unreserve(void *ptr, usize size) { return false; }

} // namespace xos::mm::sboo
