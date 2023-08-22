#include <xos/mm/kalloc.hpp>
#include <xos/mm/phymm.hpp>
#include <xos/utils/log.h>

LOG_TAG("mm/kalloc")

namespace xos::mm::kalloc {

void *malloc(usize size) { return phy::malloc(size); }
void free(void *ptr) { mm::phy::free(ptr); }

void *KallocAllocator::malloc(usize size) { return kalloc::malloc(size); }
void KallocAllocator::free(void *ptr) { kalloc::free(ptr); }

KallocAllocator kalloc_allocator = KallocAllocator();

} // namespace xos::mm::kalloc

namespace xos {

void *kzmalloc(usize size) {
	void *ptr = kmalloc(size);
	if (ptr != nullptr)
		std::memset(ptr, 0, size);
	return ptr;
}

} // namespace xos

// C++ operator new([]), operator delete([]) implementation

void *operator new(std::size_t size) { return xos::kzmalloc(size); }
void *operator new[](std::size_t size) { return xos::kzmalloc(size); }
void operator delete(void *ptr) { xos::kfree(ptr); }
void operator delete[](void *ptr) { xos::kfree(ptr); }
void operator delete(void *ptr, std::size_t sz) { xos::kfree(ptr); }
void operator delete[](void *ptr, std::size_t sz) { xos::kfree(ptr); }

void *operator new(std::size_t size, const std::nothrow_t &) noexcept {
	return xos::kzmalloc(size);
}
void *operator new[](std::size_t size, const std::nothrow_t &) noexcept {
	return xos::kzmalloc(size);
}
void operator delete(void *ptr, const std::nothrow_t &) { xos::kfree(ptr); }
void operator delete[](void *ptr, const std::nothrow_t &) { xos::kfree(ptr); }

namespace std {
const nothrow_t nothrow{};
}
