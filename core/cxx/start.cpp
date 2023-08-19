#include "cxx.hpp"
#include <xos/boot/boot.h>

extern "C" {

extern "C" const char *core_init(xos::boot::boot_info_t *bootinfo);

const char *_start(xos::boot::boot_info_t *bootinfo) {
	_init();
	__stack_chk_init(bootinfo->random);
	const char *ret = core_init(bootinfo);
	_fini();
	__cxa_finalize(nullptr);
	return ret;
}
}
