#include <cstring>
#include <xos/arch/x86/interrupt.hpp>
#include <xos/utils/log.h>
#include <xos/utils/panic.h>

LOG_TAG("x86/intr");

namespace xos::arch::x86::intr {

using namespace gate;

extern "C" {
#include <xos/arch/x86/isr.h>
}

[[gnu::aligned(0x10)]] idt_gate descriptors[] = {
#define ISR_WRAPPER(vec)                                                       \
	(interrupt() | present() | dpl(0) |                                        \
	 segment(seg_selector(GDT_INDEX_CORE_CODE, 0)) |                           \
	 offset((u32)_isr_wrapper_##vec)),
#include <xos/arch/x86/isr.h>
};

void init() {
	idtr_t ptr = {
		.limit = sizeof(descriptors) - 1,
		.base = reinterpret_cast<u32>(&descriptors),
	};
	load_idtr(&ptr);
	memset(&intr_handlers, 0, sizeof(intr_handlers));
	handlers::init();
	// disable the 8259 PIC, or else it will generate interrupt 8 for its timer
	asm volatile("movb $0xff, %%al\n\t"
				 "outb %%al, $0xa1\n\t"
				 "outb %%al, $0x21\n\t" ::
					 : "al");
	enable_intr();
}

interrupt_handler *intr_handlers[IDT_COUNT];

void x86_interrupt_handler(isr_param_t *isr) {
	if (intr_handlers[isr->vector] != nullptr)
		return intr_handlers[isr->vector](isr);

	PANIC("UNKNOWN INTERRUPT %x : CS:EIP 0x%x:0x%x\n"
		  "EAX 0x%x EBX 0x%x ECX 0x%x EDX 0x%x\n"
		  "ESI 0x%x EDI 0x%x EBP 0x%x\n"
		  "DS 0x%x ES 0x%x FS 0x%x GS 0x%x",
		  isr->vector, isr->cs, isr->eip, isr->reg.eax, isr->reg.ebx,
		  isr->reg.ecx, isr->reg.edx, isr->reg.esi, isr->reg.edi, isr->reg.ebp,
		  isr->reg.ds, isr->reg.es, isr->reg.fs, isr->reg.gs);
}

namespace handlers {
void DE(isr_param_t *isr) {
	PANIC("divide zero error at 0x%x:0x%x", isr->cs, isr->eip);
}
void DB(isr_param_t *isr) {
	PANIC("debug exception at 0x%x:0x%x", isr->cs, isr->eip);
}
void OF(isr_param_t *isr) {
	PANIC("overflow exception at 0x%x:0x%x", isr->cs, isr->eip);
}
void UD(isr_param_t *isr) {
	PANIC("undefined instruction at 0x%x:0x%x", isr->cs, isr->eip);
}
void DF(isr_param_t *isr) { PANIC("double fault (#DF)"); }
void TS(isr_param_t *isr) { PANIC("invalid TSS (#TS)"); }
void NP(isr_param_t *isr) { PANIC("segment not present (#NP)"); }
void SS(isr_param_t *isr) {
	PANIC("stack segment fault (#SS), (user) SS: 0x%x, ESP: 0x%x", isr->ss,
		  isr->reg);
}
void GP(isr_param_t *isr) {
	PANIC("general protection exception (#GP) at 0x%x:0x%x", isr->cs, isr->eip);
}
void PF(isr_param_t *isr) { PANIC("page fault (#PF)"); }
void MF(isr_param_t *isr) {
	PANIC("x87 FPU FP error (#MF) at 0x%x:0x%x", isr->cs, isr->eip);
}
void init() {
	intr_handlers[0] = &DE;
	intr_handlers[1] = &DB;
	intr_handlers[4] = &OF;
	intr_handlers[6] = &UD;
	intr_handlers[8] = &DF;
	intr_handlers[10] = &TS;
	intr_handlers[11] = &NP;
	intr_handlers[12] = &SS;
	intr_handlers[13] = &GP;
	intr_handlers[14] = &PF;
	intr_handlers[16] = &MF;
}
} // namespace handlers

} // namespace xos::arch::x86::intr
