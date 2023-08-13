#ifndef __CORE_TYPES_HEADER__
#define __CORE_TYPES_HEADER__ 1

#ifndef ASM_FILE
typedef char bool;

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
#endif

#define false (bool)0
#define true (bool)1

#define NULL (void *)0

#define U8_MIN 0x00
#define U8_MAX 0xff
#define U16_MIN 0x0000
#define U16_MAX 0xffff
#define U32_MIN 0x00000000
#define U32_MAX 0xffffffff
#define U64_MIN 0x0000000000000000
#define U64_MAX 0xffffffffffffffff

#define _unused(x) (void)(x)

#endif
