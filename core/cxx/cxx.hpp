#ifndef __CORE_CXX_CXX_HPP__
#define __CORE_CXX_CXX_HPP__

#include <types.h>

extern "C" {
// CRT
[[gnu::weak]] void _init();
[[gnu::weak]] void _fini();

// CXX ABI
/// Call atexit entries
void __cxa_finalize(void *f);
/// Pure virtual function handler
void __cxa_pure_virtual();

// SSP
/// SSP random number
extern void *__stack_chk_guard;

/// Initialize SSP
void __stack_chk_init(u64 rand);

/// SSP failure handler
[[noreturn]] void __stack_chk_fail();
}

#endif
