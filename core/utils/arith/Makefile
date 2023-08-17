compiler-rt-builtins += mulxc3.o mulsc3.o muldc3.o udivdi3.o divdi3.o umoddi3.o moddi3.o udivmoddi4.o

$(call load-objs, core, $(foreach __comprt,$(compiler-rt-builtins),builtins/$(__comprt)))

$(curout)builtins/%: CFLAGS += -D__ELF__

$(curout)arith.a: $(foreach __comprt,$(compiler-rt-builtins),$(curout)builtins/$(__comprt))
compiler-rt-builtins-lib := $(curout)arith.a
