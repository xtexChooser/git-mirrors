#ifndef __XOS_UTILS_LOG_H__
#define __XOS_UTILS_LOG_H__

#include <cstdarg>
#include <types.h>

#ifdef __cplusplus
namespace xos::log {
extern "C" {
#endif

/**
 * @brief Log level
 *
 */
#ifdef __cplusplus
enum LogLevel {
#else
enum log_level {
#endif
	KLOG_DEBUG = 0,
	KLOG_INFO,
	KLOG_WARN,
	KLOG_ERROR,
	KLOG_PANIC,
};

#ifndef __cplusplus
typedef enum log_level log_level_t;
#define LogLevel log_level_t
#endif

/**
 * @brief Print a core log message
 *
 * @param tag Tag
 * @param level Level
 * @param fmt Format
 * @param ... Arguments
 */
void kprintf(const str tag, const LogLevel level, const str fmt, ...);

/**
 * @brief Print a core log message
 *
 * @param tag Tag
 * @param level Level
 * @param fmt Format
 * @param args Arguments
 */
void kvprintf(const str tag, const LogLevel level, const str fmt,
			  std::va_list args);

#ifdef __cplusplus
}
} // namespace xos::log
#endif

/**
 * @brief Get KLog tag in current context
 * This should be defined with ::LOG_TAG macro and is file scoped
 *
 * @return str Tag
 */
static inline str klog_tag();

/// Define a log tag for current scope
#define LOG_TAG(x)                                                             \
	static inline str klog_tag() { return (char *)x; }

/// Print a log with the given level and format
#ifdef __cplusplus
#define LOG(level, fmt, ...)                                                   \
	xos::log::kprintf(klog_tag(), xos::log::level, (char *)fmt, ##__VA_ARGS__)
#else
#define LOG(level, fmt, ...) kprintf(klog_tag(), level, fmt, ##__VA_ARGS__)
#endif

/// Log a debug message
#define DEBUG(fmt, ...) LOG(KLOG_DEBUG, fmt, ##__VA_ARGS__)

/// Log a info message
#define INFO(fmt, ...) LOG(KLOG_INFO, fmt, ##__VA_ARGS__)

/// Log a warning message
#define WARN(fmt, ...) LOG(KLOG_WARN, fmt, ##__VA_ARGS__)

/// Log an error message
#define ERROR(fmt, ...) LOG(KLOG_ERROR, fmt, ##__VA_ARGS__)

#endif
