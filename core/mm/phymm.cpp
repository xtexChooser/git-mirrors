#include <algorithm>
#include <xos/math.h>
#include <xos/mm/buddy.hpp>
#include <xos/mm/phymm.hpp>
#include <xos/mm/sboo.hpp>
#include <xos/mm/slob.hpp>
#include <xos/utils/log.h>
#include <xos/utils/panic.h>

LOG_TAG("phymm");

using namespace std;

namespace xos::mm::phy {
using namespace buddy;

buddy::BuddyAllocator *main_alloc;

void init(boot::boot_info_t *bootinfo) {
	// find base address
	usize pmem_size = (usize)bootinfo->mem_upper;
	usize buddy_size = BuddyAllocator::get_size((usize)bootinfo->mem_upper);
	usize buddy_base =
		bootinfo->do_aslr
			? (usize)ceilu((usize)bootinfo->core_load_offset, SZ_4K)
			: (usize)flooru((usize)bootinfo->random, SZ_4K);
	while (1) {
		usize buddy_end = buddy_base + buddy_size;
		if (buddy_base > pmem_size)
			buddy_base -= pmem_size;
		else if (buddy_end > pmem_size)
			buddy_base -= buddy_size;
		else {
			// check reserved memory
			boot::boot_reserved_mem_t *memblock = bootinfo->reserved_mem;
			while (memblock != nullptr) {
				if (max((usize)memblock->start, buddy_base) <
					min((usize)memblock->end, buddy_end)) {
					break;
				}
				memblock = memblock->next;
			}
			if (memblock != nullptr)
				buddy_base +=
					min((usize)memblock->end - buddy_base * 2, (usize)SZ_4M);
			else
				break;
		}
	}
	INFO("main buddy base: 0x%x size: 0x%x", buddy_base, buddy_size);
	// initialize buddy alloc
	void *metadata_alloc = (void *)(buddy_base + sizeof(BuddyAllocator));
	main_alloc = new (reinterpret_cast<void *>(buddy_base))
		BuddyAllocator(pmem_size, &metadata_alloc);
	ASSERT_EQ(buddy_base + buddy_size, (usize)metadata_alloc);
	// reserve memory blocks
	reserve((void *)buddy_base, buddy_size);
	boot::boot_reserved_mem *reserved_mem = bootinfo->reserved_mem;
	while (reserved_mem != nullptr) {
		// the first page is not managed by buddy allocator
		reserve((void *)max((usize)reserved_mem->start, PAGE_SIZE),
				(usize)reserved_mem->end - (usize)reserved_mem->start);
		reserved_mem = reserved_mem->next;
	}
}

void *malloc(usize size) { return main_alloc->malloc(size); }
void free(void *ptr) { return main_alloc->free(ptr); }

void reserve(void *ptr, usize size) {
	ASSERT(main_alloc->reserve(ptr, size), "reserve phy mem: %p + %p", ptr,
		   size);
}
void unreserve(void *ptr, usize size) {
	ASSERT(main_alloc->unreserve(ptr, size), "unreserve phy mem: %p + %p", ptr,
		   size);
}
} // namespace xos::mm::phy
