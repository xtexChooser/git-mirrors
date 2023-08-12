#ifndef __CORE_UTIL_MATH_HEADER__
#define __CORE_UTIL_MATH_HEADER__ 1

#define min(a, b) ((a) < (b) ? (a) : (b))
#define max(a, b) ((a) < (b) ? (b) : (a))

#define flooru(v, m) (v - (v % m))
#define ceilu(v, m) ((v % m > 0) ? floor_unsigned(v + m) : v)

#endif
