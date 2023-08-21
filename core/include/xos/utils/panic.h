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
void kpanic(const str tag, const str fmt, ...) __attribute__((__noreturn__));

/**
 * @brief Infinity loop
 * This is platform-dependent and should be defined by arch. Because simply
 * `while (1);` is undefined behaviour.
 *
 */
void khalt() __attribute__((__noreturn__));

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

#endif
