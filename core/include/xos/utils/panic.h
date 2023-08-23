#ifndef __XOS_UTILS_PANIC_H__
#define __XOS_UTILS_PANIC_H__

#include <types.h>
#include <xos/utils/log.h>

#ifdef __cplusplus
namespace xos {
extern "C" {
#endif

/**
 * @brief Throw a core panic
 *
 * @param tag Log tag
 * @param fmt Message format string
 * @param ... Format arguments
 */
[[noreturn]] void kpanic(const str tag, const str fmt, ...);

/**
 * @brief Infinity loop
 *
 * This is architecture-dependent.
 * An example implementation is:
 * ```
 * volatile int __infloop = 1;
 * while (__infloop)
 *     ; // architecture ways to halt the processor. to save power.
 * ```
 *
 * Because simply `while (1);` (side-effect-free infinity-loop) is undefined
 * (or compiler-defined) behaviour in C (ref: WG14/N1528)
 * (defined at C23 standard(WG14/N3096,ISO/IEC 9899:2023) 6.8.5), we have to use
 * a `volatile` to force the compiler keep the loop. `[[clang::optnone]]` can be
 * used as need, too.
 *
 * @see https://github.com/Minep/lunaix-os/issues/16
 * @see https://www.open-std.org/jtc1/sc22/wg14/www/docs/n1528.htm
 *
 */
[[noreturn]] void khalt();

#ifdef __cplusplus
}
} // namespace xos
#endif

/// Throw a core panic
#ifdef __cplusplus
#define PANIC(fmt, ...) xos::kpanic(klog_tag(), (char *)fmt, ##__VA_ARGS__)
#else
#define PANIC(fmt, ...) kpanic(klog_tag(), fmt, ##__VA_ARGS__)
#endif

/// Assert
#define ASSERT(cond, fmt, ...)                                                 \
	if (!(cond)) {                                                             \
		PANIC("assertion failed: " #cond " " #fmt, ##__VA_ARGS__);             \
	}

/// Assert the condition is true
#define ASSERT_TRUE(cond) ASSERT(cond, "")
/// Assert the condition is false
#define ASSERT_FALSE(cond) ASSERT(!(cond), "")
/// Assert two numbers are equal
#define ASSERT_EQ(a, b) ASSERT((a) == (b), "%d %d", (u32)(a), (u32)(b))
/// Assert two numbers are not equal
#define ASSERT_NEQ(a, b) ASSERT((a) != (b), "%d %d", (u32)(a), (u32)(b))
/// Assert a pointer is null
#define ASSERT_NULL(cond) ASSERT((cond) == nullptr, "")
/// Assert a pointer is not null
#define ASSERT_NONNULL(cond) ASSERT((cond) != nullptr, "")

#endif
