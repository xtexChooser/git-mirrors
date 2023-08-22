#ifndef __XOS_ARCH_X86_GDT_HPP__
#define __XOS_ARCH_X86_GDT_HPP__

#include <types.h>

namespace xos::arch::x86::gdt {

typedef u64 gdt_desc;
typedef u8 gdt_type;
typedef u16 segment_selector;

namespace desc {
inline gdt_desc seg_type(gdt_type v) { return (u64)v << 8 << 32; }
inline gdt_desc code_data(bool v = true) { return (u64)(u8)v << 12 << 32; }
inline gdt_desc dpl(u8 v) { return (u64)v << 13 << 32; }
inline gdt_desc present(bool v = true) { return (u64)(u8)v << 15 << 32; }
inline gdt_desc avl(bool v = false) { return (u64)(u8)v << 20 << 32; }
inline gdt_desc bits(bool bits32, bool bits64) {
	return ((u64)(u8)bits32 << 22 << 32) | ((u64)(u8)bits64 << 21 << 32);
}
inline gdt_desc granularity(bool v = true) { return (u64)(u8)v << 23 << 32; }
inline gdt_desc base32(u32 base = 0) {
	return ((base & 0x0000ffff) << 16) |
		   (((u64)base & 0x00ff0000) >> 16 << 32) |
		   (((u64)base & 0xff000000) >> 24 << 56);
}
inline gdt_desc limit32(u32 limit = 0xfffff) {
	return (limit & 0x0ffff) | (((u64)limit & 0xf0000) >> 16 << 48);
}
} // namespace desc

namespace type {
inline gdt_type data(bool write, bool accessed = false, bool expand = false) {
	return ((u8)accessed) | ((u8)write << 1) | ((u8)expand << 2) | (0 << 3);
}
inline gdt_type code(bool read, bool conforming = false,
					 bool accessed = false) {
	return ((u8)accessed) | ((u8)read << 1) | ((u8)conforming << 2) | (1 << 3);
}
} // namespace type

#define GDT_COUNT 5
#define GDT_INDEX_NULL 0
#define GDT_INDEX_CORE_CODE 1
#define GDT_INDEX_CORE_DATA 2
#define GDT_INDEX_USER_CODE 3
#define GDT_INDEX_USER_DATA 4

inline segment_selector gdt_seg_sel(u16 index, u8 rpl) {
	return (index << 3) | (0 << 2) | (rpl & 0b11);
}

struct gdtr {
	u16 limit;
	u32 base;
} __attribute__((packed));
typedef struct gdtr gdtr_t;

extern gdt_desc descriptors[GDT_COUNT];

void init();

/**
 * @brief Set GDTR register
 *
 * @param ptr GDTR struct
 */
inline void load_gdtr(gdtr_t *ptr) { asm volatile("lgdt (%0)" : : "r"(ptr)); }

/**
 * @brief Set DS ES FS GS and SS register
 *
 * @param seg Segment selector
 */
void load_data_seg(segment_selector seg);

/**
 * @brief Set CS register
 *
 * @param seg Segment selector
 */
void load_code_seg(segment_selector seg);

} // namespace xos::arch::x86::gdt

#endif
