#ifndef __TYPES_H__
#define __TYPES_H__

#ifndef ASM_FILE

typedef unsigned char u8;
typedef char i8;
typedef unsigned short u16;
typedef short i16;
typedef unsigned int u32;
typedef int i32;
typedef unsigned long long u64;
typedef long long i64;

typedef char *str;

#define isize i32
#define usize u32

#ifndef __cplusplus
typedef char bool;

#define false (bool)0
#define true (bool)1

#define NULL (void *)0
#endif

#endif

#define U8_MIN 0x00u
#define U8_MAX 0xffu
#define U16_MIN 0x0000u
#define U16_MAX 0xffffu
#define U32_MIN 0x00000000u
#define U32_MAX 0xffffffffu
#define U64_MIN 0x0000000000000000u
#define U64_MAX 0xffffffffffffffffu

#define _unused(x) (void)(x)

#define SZ_1K 0x400u
#define SZ_2K 0x800u
#define SZ_4K 0x1000u
#define SZ_16K 0x4000u
#define SZ_32K 0x8000u
#define SZ_64K 0x10000u
#define SZ_1M 0x100000u
#define SZ_2M 0x200000u
#define SZ_4M 0x400000u

#ifdef __cplusplus
#include <cstddef>
#include <new> // placement new
#endif

#endif
