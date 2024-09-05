TOOLKIT 		?= pnpx @xvnet/registry-toolkit

.PHONY: all roa
all: dist roa

dist:
	mkdir dist

# ========== ROA ==========
ROA_GEN			= $(TOOLKIT) roa
ROA4_DEP		= route/*
ROA6_DEP		= route6/*
ROA_ALL_DEP		= $(ROA4_DEP) $(ROA6_DEP)
roa: dist/roa4.json dist/roa6.json dist/roa.json \
	dist/roa4_bird2.conf dist/roa6_bird2.conf dist/roa_bird2.conf \
	dist/roa4_bird1.conf dist/roa6_bird1.conf dist/roa_bird1.conf \
	dist/roa4_grtr.json dist/roa6_grtr.json dist/roa_grtr.json \
	dist/roa4_obgpd.conf dist/roa6_obgpd.conf dist/roa_obgpd.conf \

#     ========== JSON ==========
dist/roa4.json: $(ROA4_DEP)
	$(ROA_GEN) json ipv4 > $@
dist/roa6.json: $(ROA6_DEP)
	$(ROA_GEN) json ipv6 > $@
dist/roa.json: $(ROA_ALL_DEP)
	$(ROA_GEN) json all > $@

#     ========== BIRD2 ==========
dist/roa4_bird2.conf: $(ROA4_DEP)
	$(ROA_GEN) bird2 ipv4 > $@
dist/roa6_bird2.conf: $(ROA6_DEP)
	$(ROA_GEN) bird2 ipv6 > $@
dist/roa_bird2.conf: $(ROA_ALL_DEP)
	$(ROA_GEN) bird2 all > $@

#     ========== BIRD1 ==========
dist/roa4_bird1.conf: $(ROA4_DEP)
	$(ROA_GEN) bird1 ipv4 > $@
dist/roa6_bird1.conf: $(ROA6_DEP)
	$(ROA_GEN) bird1 ipv6 > $@
dist/roa_bird1.conf: $(ROA_ALL_DEP)
	$(ROA_GEN) bird1 all > $@

#     ========== GoRTR ==========
dist/roa4_grtr.json: $(ROA4_DEP)
	$(ROA_GEN) grtr ipv4 > $@
dist/roa6_grtr.json: $(ROA6_DEP)
	$(ROA_GEN) grtr ipv6 > $@
dist/roa_grtr.json: $(ROA_ALL_DEP)
	$(ROA_GEN) grtr all > $@

#     ========== GoRTR ==========
dist/roa4_obgpd.conf: $(ROA4_DEP)
	$(ROA_GEN) obgpd ipv4 > $@
dist/roa6_obgpd.conf: $(ROA6_DEP)
	$(ROA_GEN) obgpd ipv6 > $@
dist/roa_obgpd.conf: $(ROA_ALL_DEP)
	$(ROA_GEN) obgpd all > $@

# ========== Lint Report ==========
lint_report: dist/lint_report.txt
dist/lint_report.txt:
	touch $@
	echo Lint Report: >> $@
	$(TOOLKIT) lint >> $@
	echo -e \\n\\n\\n >> $@
	echo Format Report: >> $@
	$(TOOLKIT) format >> $@
