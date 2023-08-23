$(call load-objs, boot, boot.o)
$(call load-objs, core, init.o)
$(call load-dirs, multiboot/)

$(curout)boot.o: NO_PIE = 1
