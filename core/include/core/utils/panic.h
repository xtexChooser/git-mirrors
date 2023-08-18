#pragma once

#include <core/utils/log.h>
#include <types.h>

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
__attribute__((__noreturn__)) void kpanic(const str tag, const str fmt, ...);

#ifdef __cplusplus
}
} // namespace xos
#endif

/// Throw a core panic
#ifdef __cplusplus
#define PANIC(fmt, ...)                                                        \
	xos::kpanic(klog_tag(), (char *)fmt, ##__VA_ARGS__)
#else
#define PANIC(fmt, ...) kpanic(klog_tag(), fmt, ##__VA_ARGS__)
#endif
