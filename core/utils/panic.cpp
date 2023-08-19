#include <xos/utils/log.h>
#include <xos/utils/panic.h>

namespace xos {
void kpanic(const str tag, const str fmt, ...) {
	va_list args;
	va_start(args, fmt);
	log::kvprintf(tag, log::LogLevel::KLOG_PANIC, fmt, args);
	va_end(args);
	while (1) {
	}
}
} // namespace xos
