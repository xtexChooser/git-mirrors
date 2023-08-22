#ifndef __XOS_MM_MM_HPP__
#define __XOS_MM_MM_HPP__

#include <cstring>
#include <types.h>
#include <xos/boot/boot.h>

namespace xos::mm {

void mm_init(boot::boot_info_t *bootinfo);

class MemAllocator {
public:
	/**
	 * @brief Allocate a memory block
	 *
	 * @param size Size
	 * @return void* Pointer
	 */
	__attribute__((malloc)) virtual void *malloc(usize size) = 0;

	/**
	 * @brief Release a memory block
	 *
	 * @param ptr Pointer
	 */
	virtual void free(void *ptr) = 0;

	/**
	 * @brief Allocate a zero-filled memory block
	 *
	 * @param num Size of the array
	 * @param size Size of array element
	 * @return void* Pointer
	 */
	virtual void *calloc(usize num, usize size);

	/**
	 * @brief Change the size of a memory block
	 *
	 * @param ptr Pointer
	 * @param new_size New size
	 * @return void* New pointer
	 * @return null Unsupported or failed
	 */
	virtual void *realloc(void *ptr, usize new_size);

	/**
	 * @brief Reserve a memory block as used
	 *
	 * @param ptr Pointer
	 * @param size Size
	 * @return true Succeeded
	 * @return false Unsupported for failed
	 */
	virtual bool reserve(void *ptr, usize size);
	/**
	 * @brief Un-reserve a memory block as not used
	 *
	 * @param ptr Pointer
	 * @param size Size
	 * @return true Succeeded
	 * @return false Unsupported for failed
	 */
	virtual bool unreserve(void *ptr, usize size);
};
} // namespace xos::mm

inline void *operator new(std::size_t size, xos::mm::MemAllocator *alloc) {
	return alloc->malloc(size);
}
inline void *operator new[](std::size_t size, xos::mm::MemAllocator *alloc) {
	return alloc->malloc(size);
}
inline void operator delete[](void *ptr, xos::mm::MemAllocator *alloc) {
	alloc->free(ptr);
}
inline void operator delete(void *ptr, xos::mm::MemAllocator *alloc) {
	alloc->free(ptr);
}

#endif
