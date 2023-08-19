#include <xos/utils/panic.h>

namespace xos {

void khalt() {
	while (1) {
		asm("hlt");
	}
}

} // namespace xos
