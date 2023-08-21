#include <xos/mm/kalloc.hpp>

#include <xos/mm/phymm.hpp>

namespace xos::mm::kalloc {}

namespace xos {
__attribute__((malloc)) inline void *kmalloc(usize size) {
	return mm::phy::alloc(size);
}
__attribute__((malloc)) inline void *kzmalloc(usize size) {
	void *ptr = kmalloc(size);
	if (ptr != nullptr)
		std::memset(ptr, 0, size);
	return ptr;
}
inline void kfree(void *ptr) { mm::phy::free(ptr); }
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
