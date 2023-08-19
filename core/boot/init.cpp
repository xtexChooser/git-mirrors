#include <xos/arch/init.hpp>
#include <xos/boot/boot.h>
#include <xos/utils/log.h>
#include <xos/mm/mm.hpp>

using namespace xos::boot;
using namespace xos::init;
using namespace xos::mm;

LOG_TAG("init");

/**
 * @brief The entrypoint of core executable file
 *
 */
extern "C" const char *core_init(boot_info_t *bootinfo) {
	arch_early_init(bootinfo);
	INFO("cmdline: %s", bootinfo->cmdline);
	INFO("memory size: %dM", (u64)bootinfo->mem_upper / 1024 / 1024);
	mm_init(bootinfo);
	arch_init(bootinfo);
	return "core_init end";
}
