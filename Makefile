NAME		= xtex-os
VERSION		= 1.0

OUT			?= out
out			:= ${OUT}
SRC			?= .
src			:= ${SRC}
VPATH		+= $(src)

export NAME VERSION OUT SRC

include scripts/Makefile

$(call load-dirs,core/ arch/$(ARCH)/ external/ doc/)

include $(patsubst %.o,%.d,$(allobjs))

.PHONY: clean
clean:
	$(call action,"RM   ","$(out)/*")
	rm -rf $(out)/*

$(out)/core.o: core/linker.ld $(allobjs-core)
	$(ld) -T $< -o $@ $(filter-out $<,$^)
