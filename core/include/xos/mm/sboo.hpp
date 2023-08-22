#ifndef __XOS_MM_SBOO_HPP__
#define __XOS_MM_SBOO_HPP__

#include <xos/arch.hpp>
#include <xos/mm/mm.hpp>

/**
 * @brief The SBOO (Simple Bitmap of Objects) Allocator
 *
 */
namespace xos::mm::sboo {

#define SBOO_PAGE_MAGIC 0x43a5dc0eu

typedef u32 sboo_page_magic_t;

typedef struct sboo_pool sboo_pool_t;
struct sboo_pool {
	void *page;
	sboo_pool_t *prev;
	sboo_pool_t *next;
	bool full;
};

class SbooAllocator : public MemAllocator {
private:
	MemAllocator *arena_alloc;
	/**
	 * @brief Allocator for the bitmap, nullptr if bitmap is put after magic.
	 *
	 */
	MemAllocator *bitmap_alloc;
	sboo_pool_t *full = nullptr;
	sboo_pool_t *partial = nullptr;
	u8 first_bitmap;
	u8 first_object_offset;

public:
	const u32 objsize;
	const u32 bitmap_size;

	/**
	 * @brief Construct a new Sboo Allocator object
	 *
	 * @param arena_alloc Arena allocator
	 * @param bitmap_alloc Bitmap allocator. nullptr to let bitmap put in arena
	 * (after magic), i.e. internal bitmap
	 * @param object_size Object size
	 */
	SbooAllocator(MemAllocator *arena_alloc, MemAllocator *bitmap_alloc,
				  u32 object_size);
	~SbooAllocator();

	__attribute__((malloc)) void *malloc(usize size) override;
	void free(void *ptr) override;

	void *realloc(void *ptr, usize new_size) override;
};

} // namespace xos::mm::sboo

#endif
