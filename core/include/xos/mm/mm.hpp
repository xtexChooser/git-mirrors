#ifndef __XOS_MM_MM_HPP__
#define __XOS_MM_MM_HPP__

#include <cstring>
#include <types.h>
#include <xos/boot/boot.h>

namespace xos::mm {

void mm_init(boot::boot_info_t *bootinfo);

class MemAllocator {
public:
	virtual void *malloc(usize size) = 0;
	virtual void free(void *ptr) = 0;

	virtual void *calloc(usize num, usize size);
	virtual void *realloc(void *ptr, usize new_size);

	virtual void reserve(void *ptr, usize size);
	virtual void unreserve(void *ptr, usize size);
};
} // namespace xos::mm

#endif
