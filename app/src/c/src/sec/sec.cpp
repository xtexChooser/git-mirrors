#include "yztsec/sec.h"
#include "yztutils/arch.h"
#include <cstdlib>
#include <intrin.h>
#include <windows.h>

namespace yztsec {

/// Read PEB.BeingDebugged
bool yztsec_check_peb_debugged() {
#if defined(YZT_ENV64BIT)
  void *peb = (void *)__readgsqword(0x60);
#elif defined(YZT_ENV32BIT)
  void *peb = (void *)__readfsdword(0x30);
#endif
  // _PEB.BeingDebugged
  return *(char *)((size_t)peb + 2) == 1;
}

static bool ret;

static LONG CALLBACK yztsec_intr3_seh(_In_ PEXCEPTION_POINTERS ExceptionInfo) {
  ret = false;
  if (ExceptionInfo->ExceptionRecord->ExceptionCode == EXCEPTION_BREAKPOINT) {
    // Increase EIP/RIP to continue execution.
#ifdef _WIN64
    ExceptionInfo->ContextRecord->Rip++;
#else
    ExceptionInfo->ContextRecord->Eip++;
#endif
    return EXCEPTION_CONTINUE_EXECUTION;
  }
  return EXCEPTION_CONTINUE_SEARCH;
}

/// Check if INT3 is swallowed
bool yztsec_check_intr3() {
  PVOID seh_handle = AddVectoredExceptionHandler(1, yztsec_intr3_seh);
  ret = true;
  __debugbreak();
  bool ret = ret;
  RemoveVectoredExceptionHandler(seh_handle);
  return ret;
}

/// Check DR0-DR3 registers for Intel hardware breakpoints
bool yztsec_check_dr_regs() {
  PCONTEXT ctx = (PCONTEXT)malloc(sizeof(CONTEXT));
  SecureZeroMemory(ctx, sizeof(CONTEXT));
  ctx->ContextFlags = CONTEXT_DEBUG_REGISTERS;

  if (GetThreadContext(GetCurrentThread(), ctx)) {
    if (ctx->Dr0 != 0 || ctx->Dr1 != 0 || ctx->Dr2 != 0 || ctx->Dr3 != 0) {
      free((void *)ctx);
      return true;
    }
  }

  free((void *)ctx);
  return false;
}

static LONG CALLBACK yztsec_trap_seh(_In_ PEXCEPTION_POINTERS ExceptionInfo) {
  ret = false;
  if (ExceptionInfo->ExceptionRecord->ExceptionCode == EXCEPTION_SINGLE_STEP)
    return EXCEPTION_CONTINUE_EXECUTION;
  return EXCEPTION_CONTINUE_SEARCH;
}

/// Check if trap flag exceptions are swallowed
bool yztsec_check_trap() {
  PVOID seh_handle = AddVectoredExceptionHandler(1, yztsec_trap_seh);
  ret = true;

  __writeeflags(__readeflags() | 0x100);

  bool ret = ret;
  RemoveVectoredExceptionHandler(seh_handle);
  return ret;
}

} // namespace yztsec
