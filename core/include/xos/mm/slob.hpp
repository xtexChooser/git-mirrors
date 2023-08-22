#ifndef __XOS_MM_SLOB_HPP__
#define __XOS_MM_SLOB_HPP__

#include <xos/arch.hpp>
#include <xos/mm/mm.hpp>

/**
 * @brief The SLOB (Simple List of Block) Allocator
 *
 */
namespace xos::mm::slob {

#define SLOB_ENTRY_MAGIC 0xeffc692du

typedef struct slob_entry slob_entry_t;
struct slob_entry {
	u32 magic;
	slob_entry_t *prev;
	/**
	 * @brief The size of this block. The size is always 2-bytes aligned. And
	 * the last bit of ::size will indicate if the block is allocated.
	 *
	 */
	usize size;
	slob_entry_t *next;
};
#define SLOB_ENTRY_SIZE sizeof(xos::mm::slob::slob_entry_t)

class SlobAllocator : public MemAllocator {
private:
	slob_entry_t *first_entry = nullptr;
	MemAllocator *base_alloc;

public:
	const u32 magic;
	const usize page_size;

	SlobAllocator(MemAllocator *base, usize page_size = PAGE_SIZE,
				  u32 magic = SLOB_ENTRY_MAGIC);
	/**
	 * @brief Destroy the Slob Allocator object (unsafe)
	 * Note that this operation is unsafe if the base allocator does not support
	 * MemAllocator::unreserve operation.
	 *
	 */
	~SlobAllocator();

	__attribute__((malloc)) void *malloc(usize size) override;
	void free(void *ptr) override;

	void *realloc(void *ptr, usize new_size) override;
};

} // namespace xos::mm::slob

#endif
