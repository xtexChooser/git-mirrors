CLANG_TARGET 	= i386-pc-none-elf
MUSL_TARGET 	= i386-pc-linux-unknown
cflags			+= -m32
cflags-boot		+= -mno-avx
cflags-core		+= -mno-avx # todo: enable AVX for core
