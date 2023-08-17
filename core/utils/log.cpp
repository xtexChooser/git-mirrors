#include "core/utils/log.h"

static u16 term_x, term_y;
static void print(const str text);

namespace xos::log {
void kprintf(const str tag, const LogLevel level, const str fmt, ...) {
	print(tag);
	print((char *)": ");
	print(fmt);
	print((char *)"\n");
	return;
}
} // namespace xos::log

static void print(const str text) {
	str buf = text;
	while (*buf != 0) {
		char chr = *buf;
		if (chr == '\n' || chr == '\r') {
			term_x = 0;
			term_y = (term_y + 1) % 24;
			return;
		}
		char *vbuf = (char *)(0xB8000 + 2 * (term_x + 80 * term_y));
		*vbuf = chr;
		*(vbuf + 1) = 7;
		if (term_x == 80) {
			term_x = 0;
			term_y = (term_y + 1) % 24;
		}
		term_x += 1;
		buf++;
	}
}
