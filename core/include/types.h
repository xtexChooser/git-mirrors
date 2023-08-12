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

#endif
