#ifndef __XOS_MM_KALLOC_HPP__
#define __XOS_MM_KALLOC_HPP__

#include <types.h>
#include <xos/arch.hpp>
#include <xos/mm/mm.hpp>

/**
 * @brief The Kalloc Composite Allocator
 *
 */
namespace xos::mm::kalloc {

/**
 * @brief A Kalloc Allocator
 *
 */
class KallocAllocator : public MemAllocator {
private:
public:
	__attribute__((malloc)) void *malloc(usize size) override;
	void free(void *ptr) override;

	void *realloc(void *ptr, usize new_size) override;
};
} // namespace xos::mm::kalloc

namespace xos {
__attribute__((malloc)) inline void *kmalloc(usize size);
__attribute__((malloc)) inline void *kzmalloc(usize size);
inline void kfree(void *ptr);
} // namespace xos

#endif
