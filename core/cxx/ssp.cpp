#include "cxx.hpp"
#include "ssp_prng.h"
#include <xos/utils/panic.h>

LOG_TAG("cxx/ssp");

extern "C" {

void *__stack_chk_guard = (void *)0x5d34e8c1f9a3d4d6;

void __stack_chk_init(u64 rand) {
	__stack_chk_guard = (void *)xos_ssp_rand(rand);
}

void __stack_chk_fail() { PANIC("Stack smashing detected"); }
}
