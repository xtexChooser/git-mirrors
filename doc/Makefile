$(curout)Doxyfile: $(curdir)Doxyfile $(curdir)Doxyfile.subst
	$(call action,"GEN  ")
	cat $< > $@
	cat $(src)/doc/Doxyfile.subst | envsubst >> $@

doc: $(curout)Doxyfile
	doxygen $<

.PHONY: doc
