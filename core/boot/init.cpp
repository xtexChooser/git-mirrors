#include <core/arch/init.hpp>
#include <core/boot/boot.h>
#include <core/utils/log.h>

using namespace xos::boot;
using namespace xos::init;

LOG_TAG("init");

/**
 * @brief The entrypoint of core executable file
 *
 */
extern "C" const char *core_init(boot_info_t *bootinfo) {
	arch_early_init(bootinfo);
	// mm_init(bootinfo);
	arch_init(bootinfo);
	INFO(bootinfo->cmdline);
	INFO("test");
	return "core_init end";
}
