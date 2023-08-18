#include <core/boot/boot.h>

extern "C" {

extern void _init();
extern void _fini();
extern void __cxa_finalize(void *f);
extern "C" const char *core_init(xos::boot::boot_info_t *bootinfo);

const char *_start(xos::boot::boot_info_t *bootinfo) {
	_init();
	const char *ret = core_init(bootinfo);
	_fini();
	__cxa_finalize(nullptr);
	return ret;
}

}
