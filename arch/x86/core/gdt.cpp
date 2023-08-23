#include <xos/arch/x86/gdt.hpp>

namespace xos::arch::x86::gdt {

using namespace desc;
using namespace type;

[[gnu::aligned(0x10)]] gdt_desc descriptors[] = {
	0,
	code_data() | seg_type(code(true)) | dpl(0) | base32(0) | limit32(0xfffff) |
		present() | avl() | bits(true, false) | granularity(),
	code_data() | seg_type(data(true)) | dpl(0) | base32(0) | limit32(0xfffff) |
		present() | avl() | bits(true, false) | granularity(),
	code_data() | seg_type(code(true)) | dpl(3) | base32(0) | limit32(0xfffff) |
		present() | avl() | bits(true, false) | granularity(),
	code_data() | seg_type(data(true)) | dpl(3) | base32(0) | limit32(0xfffff) |
		present() | avl() | bits(true, false) | granularity(),
};

void init() {
	gdtr_t gdt_ptr = {
		.limit = sizeof(descriptors) - 1,
		.base = reinterpret_cast<u32>(&descriptors),
	};
	load_gdtr(&gdt_ptr);
	load_data_seg(seg_selector(GDT_INDEX_CORE_DATA, 0));
	load_code_seg(seg_selector(GDT_INDEX_CORE_CODE, 0));
}

void load_data_seg(segment_selector seg) {
	asm volatile("movw %%cx, %%es\n\t"
				 "movw %%cx, %%ds\n\t"
				 "movw %%cx, %%fs\n\t"
				 "movw %%cx, %%gs\n\t"
				 "movw %%cx, %%ss\n\t"
				 :
				 : "c"(seg));
}

void load_code_seg(segment_selector seg) {
	asm volatile("pushl %0\n\t"
				 ".byte 0xe8\n\t.long 0\n\t"	 // CALL rel32
				 ".byte 0x83, 0x04, 0x24, 5\n\t" // ADD DWORD [ESP], 5
				 "lret\n\t"
				 :
				 : "r"((u32)seg));
}

} // namespace xos::arch::x86::gdt
