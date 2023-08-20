#ifndef __XOS_MM_SLOB_HPP__
#define __XOS_MM_SLOB_HPP__

#include <xos/mm/mm.hpp>

namespace xos::mm::slob {

typedef struct slob_entry slob_entry_t;
struct slob_entry {
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
	SlobAllocator(MemAllocator *base);
	/**
	 * @brief Destroy the Slob Allocator object (unsafe)
	 * Note that this operation is unsafe if the base allocator does not support
	 * MemAllocator::unreserve operation.
	 *
	 */
	~SlobAllocator();

	void *malloc(usize size) override;
	void free(void *ptr) override;

	void *realloc(void *ptr, usize new_size) override;

	bool reserve(void *ptr, usize size) override;
	bool unreserve(void *ptr, usize size) override;
};

} // namespace xos::mm::slob

#endif
