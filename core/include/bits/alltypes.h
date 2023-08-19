#ifndef __BITS_ALLTYPES_H__
#define __BITS_ALLTYPES_H__

#define __LITTLE_ENDIAN 1234
#define __BIG_ENDIAN 4321
#define __USE_TIME_BITS64 1

typedef unsigned _Addr size_t;
typedef unsigned _Addr uintptr_t;
typedef _Addr ptrdiff_t;
typedef _Addr ssize_t;
typedef _Addr intptr_t;
typedef _Addr regoff_t;
typedef _Reg register_t;
typedef _Int64 time_t;
typedef _Int64 suseconds_t;

typedef signed char int8_t;
typedef signed short int16_t;
typedef signed int int32_t;
typedef signed _Int64 int64_t;
typedef signed _Int64 intmax_t;
typedef unsigned char uint8_t;
typedef unsigned short uint16_t;
typedef unsigned int uint32_t;
typedef unsigned _Int64 uint64_t;
typedef unsigned _Int64 u_int64_t;
typedef unsigned _Int64 uintmax_t;

typedef unsigned mode_t;
typedef unsigned _Reg nlink_t;
typedef _Int64 off_t;
typedef unsigned _Int64 ino_t;
typedef unsigned _Int64 dev_t;
typedef long blksize_t;
typedef _Int64 blkcnt_t;
typedef unsigned _Int64 fsblkcnt_t;
typedef unsigned _Int64 fsfilcnt_t;

typedef unsigned wint_t;
typedef unsigned long wctype_t;

typedef void *timer_t;
typedef int clockid_t;
typedef long clock_t;
struct timeval {
	time_t tv_sec;
	suseconds_t tv_usec;
};
struct timespec {
	time_t tv_sec;
	int : 8 * (sizeof(time_t) - sizeof(long)) * (__BYTE_ORDER == 4321);
	long tv_nsec;
	int : 8 * (sizeof(time_t) - sizeof(long)) * (__BYTE_ORDER != 4321);
};

typedef int pid_t;
typedef unsigned id_t;
typedef unsigned uid_t;
typedef unsigned gid_t;
typedef int key_t;
typedef unsigned useconds_t;

#undef _Addr
#undef _Int64
#undef _Reg

#endif
