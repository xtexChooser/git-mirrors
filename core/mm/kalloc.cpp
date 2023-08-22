#include <xos/arch/mm.hpp>
#include <xos/math.h>
#include <xos/mm/kalloc.hpp>
#include <xos/mm/phymm.hpp>
#include <xos/mm/sboo.hpp>
#include <xos/mm/slob.hpp>
#include <xos/utils/log.h>

LOG_TAG("mm/kalloc")

namespace xos::mm::kalloc {

using namespace slob;
using namespace sboo;

SlobAllocator *slob_alloc = nullptr;
u8 slob_allocator_place[sizeof(slob::SlobAllocator)];
SbooAllocator *sboo_alloc[KALLOC_SBOO_SIZE];

void init() {
	slob_alloc = new (&slob_allocator_place)
		SlobAllocator(phy::main_alloc, KALLOC_SLOB_PAGE_SIZE);
	for (int i = 0; i < KALLOC_SBOO_SIZE; i++) {
		sboo_alloc[i] = new (slob_alloc) SbooAllocator(
			phy::main_alloc, slob_alloc, 2 << i, SBOO_PAGE_MAGIC + i);
	}
}

void *malloc(usize size) {
	if (size == 0)
		return nullptr;
	if (size < (1 << KALLOC_SBOO_SIZE)) {
		int order = 0;
		while (size != 0) {
			order++;
			size >>= 1;
		}
		return sboo_alloc[order - 1]->malloc(size);
	} else if (size <= ((KALLOC_SLOB_PAGE_SIZE / 2) - SLOB_ENTRY_SIZE))
		return slob_alloc->malloc(size);
	return phy::malloc(size);
}
void free(void *ptr) {
	sboo_page_magic_t *magic =
		(sboo_page_magic_t *)flooru((usize)ptr, PAGE_SIZE);
	if ((*magic & ~0xff) == (SBOO_PAGE_MAGIC & ~0xff) &&
		*magic > SBOO_PAGE_MAGIC && *magic < SBOO_PAGE_MAGIC + KALLOC_SBOO_SIZE)
		return sboo_alloc[*magic - SBOO_PAGE_MAGIC]->free(ptr);

	if (((slob_entry_t *)((usize)ptr - SLOB_ENTRY_SIZE))->magic ==
		SLOB_ENTRY_MAGIC)
		return slob_alloc->free(ptr);

	mm::phy::free(ptr);
}

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
