#ifndef __XOS_MM_SBOO_HPP__
#define __XOS_MM_SBOO_HPP__

#include <xos/arch.hpp>
#include <xos/mm/mm.hpp>

/**
 * @brief The SBOO (Simple Bitmap of Objects) Allocator
 *
 */
namespace xos::mm::sboo {

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
	MemAllocator *bitmap_alloc;
	sboo_pool_t *full = nullptr;
	sboo_pool_t *partial = nullptr;
	u8 first_bitmap;
	u8 first_object_offset;

public:
	const u32 objsize;
	const u32 bitmap_size;
	const usize page_size;

	SbooAllocator(MemAllocator *arena_alloc, MemAllocator *bitmap_alloc,
				  u32 object_size, usize page_size = PAGE_SIZE);
	~SbooAllocator();

	__attribute__((malloc)) void *malloc(usize size) override;
	void free(void *ptr) override;

	void *realloc(void *ptr, usize new_size) override;
};

} // namespace xos::mm::sboo

#endif
