compiler-rt-builtins-src:=$(curdir)builtins/
compiler-rt-builtins-out:=$(curout)builtins/

compiler-rt-builtins-objs += udivdi3.o divdi3.o umoddi3.o moddi3.o udivmoddi4.o

$(call load-objs, core, $(foreach __comprt,$(compiler-rt-builtins-objs),builtins/$(__comprt)))

$(compiler-rt-builtins-out)%: CFLAGS += -D__ELF__
