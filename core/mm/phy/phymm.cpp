#include <xos/math.h>
#include <xos/mm/phy/buddy.hpp>
#include <xos/mm/phy/phymm.hpp>
#include <xos/utils/log.h>
#include <xos/utils/panic.h>

LOG_TAG("phymm");

namespace xos::mm::phy {
using namespace buddy;

buddy::BuddyAllocator *main_alloc;

void phymm_init(boot::boot_info_t *bootinfo) {
	// find base address
	usize pmem_size = (usize)bootinfo->mem_upper;
	usize buddy_size = BuddyAllocator::get_size((usize)bootinfo->mem_upper);
	usize buddy_base = (usize)ceilu((usize)bootinfo->core_load_offset, SZ_4K);
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
				buddy_base += min((usize)memblock->end - buddy_base * 2, SZ_4M);
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
	for (int i = 0; i < 10; i++) {
		INFO("alloc 4k: 0x%x", main_alloc->alloc(SZ_4K));
	}
}
} // namespace xos::mm::phy
