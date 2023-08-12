#include "arch/boot.h"
#include "boot/boot.h"
#include "multiboot.h"
#include "types.h"
#include <stdarg.h>

#define CHECK_MB_FLAG(flags, bit) ((flags) & (1 << (bit)))

#define TEXT_VIDEO_BUFFER 0xB8000

static unsigned short text_x_pos = 0, text_y_pos = 0;
static unsigned int text_width = 0, text_height = 0;
static bool text_mode;

void cmain(unsigned long magic, multiboot_info_t *mbi);
void clear();
void putchar(char chr);
void print(char *str);

void cmain(unsigned long magic, multiboot_info_t *mbi) {
	// check bootloader magic
	if (magic != MULTIBOOT_BOOTLOADER_MAGIC) {
		text_mode = true; // assume EGA text buffer available
		print("multiboot: boot: invalid magic number\n");
		return;
	}

	// init EGA text buffer
	text_mode = (mbi->flags & MULTIBOOT_INFO_FRAMEBUFFER_INFO >> 12) &&
				mbi->framebuffer_type == MULTIBOOT_FRAMEBUFFER_TYPE_EGA_TEXT;
	text_width = mbi->framebuffer_width;
	text_height = mbi->framebuffer_height;
	clear();

	// boot
	arch_boot();
	// reserve memories
	do_core_boot();
}

void clear() {
	if (!text_mode)
		return;
	char *buf = (char *)TEXT_VIDEO_BUFFER;
	char *end_buf = buf + 2 * text_width * text_height;
	while (buf < end_buf) {
		*buf = ' ';
		*(buf + 1) = 7;
		buf += 2;
	}
}

void print(char *str) {
	if (!text_mode)
		return;
	while (*str != 0) {
		putchar(*str);
		str++;
	}
}

void putchar(char chr) {
	if (chr == '\n' || chr == '\r') {
		text_x_pos = 0;
		text_y_pos = (text_y_pos + 1) % text_height;
		return;
	}
	char *buf = (char *)(TEXT_VIDEO_BUFFER +
						 2 * (text_x_pos + text_width * text_y_pos));
	*buf = chr;
	*(buf + 1) = 7;
	if (text_x_pos == text_width) {
		text_x_pos = 0;
		text_y_pos = (text_y_pos + 1) % text_height;
	}
	text_x_pos += 1;
}
