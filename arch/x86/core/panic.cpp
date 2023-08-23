#include <xos/utils/panic.h>

namespace xos {

[[clang::optnone]] void khalt() {
	// see also: https://github.com/Minep/lunaix-os/issues/16
	asm volatile("cli");
	volatile int __infloop = 1;
	while (__infloop)
		asm("hlt");
	while (1)
		asm("hlt");
}

} // namespace xos
