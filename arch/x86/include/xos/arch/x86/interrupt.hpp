#ifndef __XOS_ARCH_X86_INTERRUPT_HPP__
#define __XOS_ARCH_X86_INTERRUPT_HPP__

#include <types.h>
#include <xos/arch/x86/gdt.hpp>

namespace xos::arch::x86::intr {

typedef u64 idt_gate;

namespace gate {
inline idt_gate task() { return (u64)0b00101 << 8 << 32; }
inline idt_gate interrupt(bool bits32 = true) {
	return (((u64)(u8)bits32 << 6) | 0b00110000) << 5 << 32;
}
inline idt_gate trap(bool bits32 = true) {
	return (((u64)(u8)bits32 << 6) | 0b00111000) << 5 << 32;
}

inline idt_gate present(bool v = true) { return (u64)(u8)v << 15 << 32; }
inline idt_gate dpl(u8 v) { return (u64)v << 13 << 32; }

inline idt_gate offset(u32 offset) {
	return (offset & 0x0000ffff) |
		   (((u64)offset & 0xffff0000) >> 16 << 16 << 32);
}
inline idt_gate segment(gdt::segment_selector seg) { return (u64)(seg << 16); }
using gdt::seg_selector;
} // namespace gate

#define IDT_COUNT 256

struct idtr {
	u16 limit;
	u32 base;
} __attribute__((packed));
typedef struct idtr idtr_t;

[[gnu::aligned(0x10)]] extern idt_gate descriptors[IDT_COUNT];

void init();

/**
 * @brief Set IDTR register
 *
 * @param ptr IDTR struct
 */
inline void load_idtr(idtr_t *ptr) { asm volatile("lidt (%0)" : : "r"(ptr)); }

/**
 * @brief Enable maskable interrupts
 */
inline void enable_intr() { asm volatile("sti"); }

/**
 * @brief Disable maskable interrupts
 */
inline void disable_intr() { asm volatile("cli"); }

extern "C" {
struct [[gnu::packed]] isr_param {
	struct [[gnu::packed]] {
		u32 ebp;
		u32 edi;
		u32 esi;
		u32 edx;
		u32 ecx;
		u32 ebx;
		u32 eax;
		u32 ds;
		u32 es;
		u32 fs;
		u32 gs;
		u32 esp;
	} reg;

	u32 vector;
	u32 error_code;
	u32 eip;
	u32 cs;
	u32 eflags;
	u32 esp;
	u32 ss;
};
typedef struct isr_param isr_param_t;
typedef void interrupt_handler(isr_param_t *isr);

extern interrupt_handler *intr_handlers[IDT_COUNT];

[[gnu::regparm(1)]] void x86_interrupt_handler(isr_param_t *isr);
}

namespace handlers {
void init();
}

} // namespace xos::arch::x86::intr

#endif
