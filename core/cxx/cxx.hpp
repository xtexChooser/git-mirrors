#ifndef __CORE_CXX_CXX_HPP__
#define __CORE_CXX_CXX_HPP__

#include <types.h>

extern "C" {
// CRT
void _init();
void _fini();

// CXA
/// Call atexit entries
void __cxa_finalize(void *f);

// SSP
/// SSP random number
extern void *__stack_chk_guard;

/// Initialize SSP
void __stack_chk_init(u64 rand);

/// SSP failure handler
void __stack_chk_fail() __attribute__((noreturn));
}

#endif
