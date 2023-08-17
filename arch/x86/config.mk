CLANG_TARGET 	= i386-pc-none-elf
MUSL_TARGET 	= i386-pc-linux-unknown
cflags			+= -m32
cflags-boot		+= -mno-avx -mno-avx512f -mno-avx2 -mno-mmx -mno-sse
cflags-core		+= -mno-avx -mno-avx512f -mno-avx2 -mno-mmx -mno-sse # todo: enable AVX for core
