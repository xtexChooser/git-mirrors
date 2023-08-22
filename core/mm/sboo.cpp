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
	if (sizeof(sboo_page_magic_t) + sizeof(sboo_pool) + bitmap_size < objsize)
		this->bitmap_alloc = nullptr;

	first_bitmap = 1; // for the first object
	usize header_size;
	if (this->bitmap_alloc == nullptr)
		header_size = sizeof(sboo_page_magic_t) + sizeof(sboo_pool);
	else
		header_size = sizeof(sboo_page_magic_t) + sizeof(sboo_pool *);
	usize bits = std::max(ceilu(header_size, objsize) / objsize, 1u);
	first_object_offset = bits * objsize;
	for (; bits > 0; bits--) {
		first_bitmap <<= 1;
		first_bitmap |= 1;
	}
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
				pool->prev = nullptr;
				partial->prev = nullptr;
				full->prev = pool;
				full = pool;
			}
		}
		return reinterpret_cast<void *>((usize)pool->page +
										objsize * (offset * 8 + bit));
	} else {
		// alloc new page
		void *page = arena_alloc->malloc(PAGE_SIZE);
		if (page == nullptr)
			return nullptr;
		*(sboo_page_magic_t *)page = SBOO_PAGE_MAGIC;
		if (bitmap_alloc != nullptr) {
			// external bitmap
			partial = pool = reinterpret_cast<sboo_pool_t *>(
				bitmap_alloc->malloc(sizeof(sboo_pool_t) + bitmap_size));
			if (pool == nullptr)
				return nullptr;
			*(sboo_pool_t **)((usize)page + sizeof(sboo_page_magic_t)) = pool;
		} else {
			// internal bitmap
			partial = pool = reinterpret_cast<sboo_pool_t *>(
				(usize)page + sizeof(sboo_page_magic_t));
		}
		pool->full = false;
		pool->next = nullptr;
		pool->prev = nullptr;
		pool->page = page;
		usize bitmap = (usize)pool + sizeof(sboo_pool);
		memset((void *)bitmap, 0, bitmap_size);

		*(u8 *)bitmap = first_bitmap;
		return reinterpret_cast<void *>((usize)page + first_object_offset);
	}
}

void SbooAllocator::free(void *ptr) {
	sboo_page_magic_t *magic =
		(sboo_page_magic_t *)flooru((usize)ptr, PAGE_SIZE);
	ASSERT_EQ(*magic, SBOO_PAGE_MAGIC);
	sboo_pool_t *pool;
	if (bitmap_alloc != nullptr)
		pool = *(sboo_pool_t **)((usize)magic + sizeof(sboo_page_magic_t));
	else
		pool = (sboo_pool_t *)((usize)magic + sizeof(sboo_page_magic_t));
	usize offset = ((usize)ptr % PAGE_SIZE) / objsize;
	u8 *bitmap = (u8 *)((usize)pool + sizeof(sboo_pool) + (offset / 8));
	*bitmap &= ~(1 << (offset % 8));
	if (pool->full) {
		// move to partial list
		pool->full = false;
		pool->prev->next = pool->next;
		pool->next->prev = pool->prev;
		pool->prev = nullptr;
		pool->next = partial;
		partial->prev = pool;
		partial = pool;
	}
}

void *SbooAllocator::realloc(void *ptr, usize new_size) {
	if (ptr == nullptr)
		return malloc(new_size);
	if (new_size == 0) {
		free(ptr);
		return nullptr;
	}
	if (new_size >= objsize)
		return nullptr;
	return ptr;
}

} // namespace xos::mm::sboo
