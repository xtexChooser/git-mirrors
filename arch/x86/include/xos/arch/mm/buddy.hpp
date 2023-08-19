#ifndef __XOS_ARCH_MM_BUDDY_HPP__
#define __XOS_ARCH_MM_BUDDY_HPP__

#include <types.h>

#define BUDDY_ALLOC_ORDERS 4
#define BUDDY_ALLOC_ORDER_SIZE                                                 \
	{ SZ_4K, SZ_16K, SZ_64K, SZ_1M }

namespace xos::mm::phy::buddy {}

#endif
