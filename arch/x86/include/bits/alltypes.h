#ifdef DOXYGEN
#define __ARCH_BITS_ALLTYPES_H__
#endif

#ifndef __ARCH_BITS_ALLTYPES_H__
#define __ARCH_BITS_ALLTYPES_H__

#define _REDIR_TIME64 1
#define _Addr int
#define _Int64 long long
#define _Reg int

#define __BYTE_ORDER 1234
#define __LONG_MAX 0x7fffffffL

#ifndef __cplusplus
#ifdef __WCHAR_TYPE__
typedef __WCHAR_TYPE__ wchar_t;
#else
typedef long wchar_t;
#endif
#endif

#if defined(__FLT_EVAL_METHOD__) && __FLT_EVAL_METHOD__ == 0
typedef float float_t;
typedef double double_t;
#else
typedef long double float_t;
typedef long double double_t;
#endif

#include_next <bits/alltypes.h>

#endif
