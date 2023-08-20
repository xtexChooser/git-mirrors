#include <xos/mm/mm.hpp>
#include <xos/mm/phymm.hpp>
#include <xos/utils/log.h>

LOG_TAG("mm");

namespace xos::mm {

void mm_init(boot::boot_info_t *bootinfo) { phy::init(bootinfo); }

void *MemAllocator::calloc(usize num, usize size) {
	void *ptr = malloc(num * size);
	std::memset(ptr, 0, num * size);
	return ptr;
}

void *MemAllocator::realloc(void *ptr, usize new_size) { return nullptr; }

void MemAllocator::reserve(void *ptr, usize size) {}
void MemAllocator::unreserve(void *ptr, usize size) {}

} // namespace xos::mm
