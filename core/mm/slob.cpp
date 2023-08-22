#include <xos/arch.hpp>
#include <xos/math.h>
#include <xos/mm/slob.hpp>
#include <xos/utils/log.h>
#include <xos/utils/panic.h>

LOG_TAG("mm/slob");

namespace xos::mm::slob {

SlobAllocator::SlobAllocator(MemAllocator *base, usize page_size, u32 magic)
	: base_alloc(base), magic(magic), page_size(page_size) {}

SlobAllocator::~SlobAllocator() {
	slob_entry_t *entry = first_entry;
	usize page = 0;
	while (entry != nullptr) {
		usize p0 = (usize)entry / page_size;
		if (p0 != page) {
			if (page != 0)
				if (!base_alloc->unreserve((void *)(page * page_size),
										   page_size))
					base_alloc->free((void *)(page * page_size));
			page = p0;
		}
		entry = entry->next;
	}
	if (page != 0)
		if (!base_alloc->unreserve((void *)(page * page_size), page_size))
			base_alloc->free((void *)(page * page_size));
}

/// @todo: spin lock
void *SlobAllocator::malloc(u32 size) {
	size = (size + 1) & ~1; // ceil to 2
	slob_entry_t *entry = first_entry;
	while (entry != nullptr) {
		ASSERT_EQ(entry->magic, this->magic);
		if ((entry->size & 1) == 0 && entry->size >= size) {
			if (entry->size == size ||
				(entry->size - size) < (SLOB_ENTRY_SIZE + 4)) {
				entry->size |= 1;
			} else {
				// split
				usize orig_size = entry->size;
				entry->size = size | 1;
				slob_entry_t *next_entry =
					(slob_entry_t *)((usize)entry + SLOB_ENTRY_SIZE + size);
				next_entry->magic = this->magic;
				next_entry->prev = entry;
				next_entry->size = orig_size - size - SLOB_ENTRY_SIZE;
				next_entry->next = entry->next;
				entry->next = next_entry;
				next_entry->next->prev = next_entry;
			}
			return (void *)((usize)entry + SLOB_ENTRY_SIZE);
		}
		entry = entry->next;
	}
	// alloc new page
	usize alloc_size = ceilu(size + SLOB_ENTRY_SIZE, page_size);
	void *new_pg = base_alloc->malloc(alloc_size);
	if (new_pg == nullptr)
		return nullptr;
	slob_entry_t *first_entry = (slob_entry_t *)new_pg;
	first_entry->magic = this->magic;
	first_entry->prev = nullptr;
	first_entry->size = size | 1;
	if (alloc_size > SLOB_ENTRY_SIZE * 2 + size) {
		slob_entry_t *second_entry =
			(slob_entry_t *)((usize)new_pg + SLOB_ENTRY_SIZE + size);
		second_entry->magic = this->magic;
		first_entry->next = second_entry;
		second_entry->prev = first_entry;
		second_entry->size = (alloc_size - (SLOB_ENTRY_SIZE * 2) - size) & ~1;
		second_entry->next = this->first_entry;
	} else
		first_entry->next = this->first_entry;
	this->first_entry = first_entry;
	return (void *)((usize)first_entry + SLOB_ENTRY_SIZE);
}

void SlobAllocator::free(void *ptr) {
	// mark as unused
	slob_entry_t *entry = (slob_entry_t *)((usize)ptr - SLOB_ENTRY_SIZE);
	ASSERT_EQ(entry->magic, this->magic);
	entry->size &= ~1;
	// merge upper
	if ((usize)entry->next == (usize)ptr + entry->size &&
		(entry->next->size & 1) == 0) {
		// next entry is just after the current entry and is not allocated
		slob_entry_t *next = entry->next;
		next->magic = 0;
		entry->next = next->next;
		entry->next->prev = entry;
		entry->size += next->size + SLOB_ENTRY_SIZE;
	}
	// merge lower
	if ((usize)entry->prev + SLOB_ENTRY_SIZE + entry->prev->size ==
			(usize)ptr &&
		(entry->prev->size & 1) == 0) {
		// last entry is just before the current entry and is not allocated
		slob_entry_t *prev = entry->prev;
		prev->next = entry->next;
		entry->next->prev = prev;
		prev->size += entry->size + SLOB_ENTRY_SIZE;
		entry->magic = 0;
	}
}

void *SlobAllocator::realloc(void *ptr, usize new_size) {
	if (ptr == nullptr)
		return malloc(new_size);
	if (new_size == 0) {
		free(ptr);
		return nullptr;
	}
	slob_entry_t *entry = (slob_entry_t *)((usize)ptr - SLOB_ENTRY_SIZE);
	usize size = entry->size & ~1;
	if (size == new_size)
		return ptr;
	else if (size > new_size) {
		if (size - new_size > SLOB_ENTRY_SIZE)
			return ptr;
		// split and call free with new entry
		slob_entry_t *next_entry = (slob_entry_t *)((usize)ptr + new_size);
		next_entry->magic = this->magic;
		entry->size = size | 1;
		next_entry->prev = entry;
		next_entry->next = entry->next;
		entry->next->prev = next_entry;
		entry->next = next_entry;
		next_entry->size = (size - new_size - SLOB_ENTRY_SIZE) | 1;
		free(next_entry);
		return ptr;
	}
	// larger size is not implemented
	return nullptr;
}

} // namespace xos::mm::slob
