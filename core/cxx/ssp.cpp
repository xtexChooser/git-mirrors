#include <core/utils/panic.h>
#include <stdint.h>
#include <types.h>

LOG_TAG("cxx/ssp");

extern "C" {

#define STACK_CHK_GUARD 0x595e9fbd94fda766
// todo: fix ssp
/*
__attribute__((weak, section(".data"))) uintptr_t __stack_chk_guard;

void __stack_chk_init(void) { __stack_chk_guard = (uintptr_t)STACK_CHK_GUARD; }

__attribute__((noreturn)) void __stack_chk_fail() {
	PANIC("Stack smashing detected");
}*/
}
