#ifndef __XOS_MM_KALLOC_HPP__
#define __XOS_MM_KALLOC_HPP__

#include <types.h>
#include <xos/arch.hpp>
#include <xos/mm/mm.hpp>

/**
 * @brief General-Purpose Kernel Memory Allocator
 *
 */
namespace xos::mm::kalloc {

void init();
void *malloc(usize size) __attribute__((malloc));
void free(void *ptr);

/**
 * @brief MemAllocator wrapper for kalloc
 * @see kalloc_allocator
 *
 */
class KallocAllocator : public MemAllocator {
public:
	__attribute__((malloc)) void *malloc(usize size) override;
	void free(void *ptr) override;
};

/**
 * @brief The global instance for KallocAllocator
 *
 */
extern KallocAllocator kalloc_allocator;

} // namespace xos::mm::kalloc

namespace xos {

__attribute__((malloc)) inline void *kmalloc(usize size) {
	return mm::kalloc::malloc(size);
}
inline void kfree(void *ptr) { mm::kalloc::free(ptr); }

__attribute__((malloc)) void *kzmalloc(usize size);

} // namespace xos

#endif
