#pragma once

#if _WIN32 || _WIN64
#if _WIN64
#define YZT_ENV64BIT
#else
#define YZT_ENV32BIT
#endif
#else
#error Neither _WIN32 nor _WIN64 is defined.
#endif
