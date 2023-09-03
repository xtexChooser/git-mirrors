default: src/gen/about-cmn.html src/gen/about-cmn-noid.html \
	src/gen/about-eng.html src/gen/about-eng-noid.html \
	src/gen/about-ext.html src/gen/about-ext-noid.html \
	src/gen/xrd.html src/gen/xrd-host-meta.xml src/gen/xrd-host-meta.json

.PHONY: default

$(shell mkdir -p src/gen)

src/gen/%.html: src/doc/%.md
	@echo "Markdown to HTML: $@"
	@pandoc --from=markdown --to=html --strip-comments --section-divs --output $@ $<

src/gen/%-noid.html: src/gen/%.html
	@echo "Remove id fields in HTML: $@"
	@sed -e 's/\sid=".*"//g' $< > $@

src/gen/xrd.html: src/includes/xrd.txt build/xrd-to-html.txt
	@echo "Generate HTML XRD"
	@sed -E -f build/xrd-to-html.txt $< > $@

src/gen/xrd-host-meta.xml: src/includes/xrd.txt build/xrd-to-host-meta.txt
	@echo "Generate RFC-6415 host-meta XML"
	@sed -E -f build/xrd-to-host-meta.txt $< > $@

src/gen/xrd-host-meta.json: src/includes/xrd.txt build/xrd-to-host-meta-json.txt
	@echo "Generate RFC-6415 host-meta JSON"
	@jo -p links=$$(jo -a $$(sed -E -f build/xrd-to-host-meta-json.txt $< | jq -c)) > $@
