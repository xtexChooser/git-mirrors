#include <stdarg.h>
#include <xos/utils/log.h>

static u16 term_x = 0, term_y = 0;
static void printf(const str fmt, ...);
static void vprintf(const str fmt, va_list valist);
static void putchar(int chr);

namespace xos::log {
static const char(level_text[])[6] = {"DEBUG", "INFO", "WARN", "ERROR",
									  "PANIC"};
void kprintf(const str tag, const LogLevel level, const str fmt, ...) {
	va_list args;
	va_start(args, fmt);
	kvprintf(tag, level, fmt, args);
	va_end(args);
}
void kvprintf(const str tag, const LogLevel level, const str fmt,
			  std::va_list args) {
	printf((char *)"%s: %s: ", level_text[level], tag);
	vprintf(fmt, args);
	putchar('\n');
	return;
}
} // namespace xos::log

static void itoa(char *buf, int base, int d) {
	char *p = buf;
	char *p1, *p2;
	unsigned long ud = d;
	int divisor = 10;

	/* If %d is specified and D is minus, put ‘-’ in the head. */
	if (base == 'd' && d < 0) {
		*p++ = '-';
		buf++;
		ud = -d;
	} else if (base == 'x')
		divisor = 16;

	/* Divide UD by DIVISOR until UD == 0. */
	do {
		int remainder = ud % divisor;

		*p++ = (remainder < 10) ? remainder + '0' : remainder + 'a' - 10;
	} while (ud /= divisor);

	/* Terminate BUF. */
	*p = 0;

	/* Reverse BUF. */
	p1 = buf;
	p2 = p - 1;
	while (p1 < p2) {
		char tmp = *p1;
		*p1 = *p2;
		*p2 = tmp;
		p1++;
		p2--;
	}
}

static void putchar(int chr) {
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
}

static void printf(const str fmt, ...) {
	va_list valist;
	va_start(valist, fmt);
	vprintf(fmt, valist);
	va_end(valist);
}

static void vprintf(str fmt, va_list valist) {
	int c;
	char buf[20];

	while ((c = *fmt++) != 0) {
		if (c != '%')
			putchar(c);
		else {
			char *p, *p2;
			int pad0 = 0, pad = 0;

			c = *fmt++;
			if (c == '0') {
				pad0 = 1;
				c = *fmt++;
			}

			if (c >= '0' && c <= '9') {
				pad = c - '0';
				c = *fmt++;
			}

			switch (c) {
			case 'd':
			case 'u':
			case 'x':
				itoa(buf, c, va_arg(valist, int));
				p = buf;
				goto string;
				break;

			case 's':
				p = va_arg(valist, char *);
				if (!p)
					p = (char *)"(null)";

			string:
				for (p2 = p; *p2; p2++)
					;
				for (; p2 < p + pad; p2++)
					putchar(pad0 ? '0' : ' ');
				while (*p)
					putchar(*p++);
				break;

			default:
				putchar(va_arg(valist, int));
				break;
			}
		}
	}
}