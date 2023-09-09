default: src/gen/about-cmn.html src/gen/about-cmn-noid.html \
	src/gen/about-eng.html src/gen/about-eng-noid.html \
	src/gen/about-ext.html src/gen/about-ext-noid.html \
	src/gen/xrd.html src/gen/host-meta.xml src/gen/host-meta.json \
	src/gen/host-webfinger.json

.PHONY: default

$(shell mkdir -p src/gen)

JSONMIN = jq -c
JSONFMT = jq --tab

src/gen/%.html: src/doc/%.md
	@echo "Markdown to HTML: $@"
	@pandoc --from=markdown --to=html --strip-comments --section-divs --output $@ $<

src/gen/%-noid.html: src/gen/%.html
	@echo "Remove id fields in HTML: $@"
	@sed -e 's/\sid=".*"//g' $< > $@

src/gen/xrd.html: src/includes/xrd.txt build/xrd-to-html.txt
	@echo "Generate HTML XRD"
	@sed -E -f build/xrd-to-html.txt $< > $@

src/gen/host-meta.xml: src/includes/xrd.txt build/xrd-to-host-meta.txt
	@echo "Generate RFC-6415 host-meta XML"
	@sed -E -f build/xrd-to-host-meta.txt $< > $@

src/gen/host-jrd.json: src/includes/xrd.txt build/xrd-to-host-meta-json.txt
	@echo "Generate host JRD"
	@jo -a $$(sed -E -f build/xrd-to-host-meta-json.txt $< | $(JSONMIN)) | $(JSONFMT) > $@

src/gen/host-meta.json: src/gen/host-jrd.json
	@echo "Generate RFC-6415 host-meta JSON"
	@jo -p links=$$(cat src/gen/host-jrd.json | $(JSONMIN)) | $(JSONFMT) > $@

src/gen/host-webfinger.json: src/gen/host-jrd.json
	@echo "Generate host WebFinger JRD"
	@jo -p subject="https://host@xtexx.eu.org" links=$$(cat src/gen/host-jrd.json | $(JSONMIN)) | $(JSONFMT) > $@
