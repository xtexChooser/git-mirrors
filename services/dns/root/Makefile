.DELETE_ON_ERROR:
SHELL := bash -c

curl = curl -q -o $@
cleanzone = ldns-read-zone -e RRSIG -e ZONEMD -e NSEC -e SOA

$(shell mkdir -p out)

all: out/root.zone.signed out/root.hint

out/root.zone: root.zone \
	out/iana-strip.zone \
	out/dn42-delegation.zone \
	out/opennic-glue-strip.zone out/opennic-delegation.zone \
	out/furnic.zone
	cat $^ | ldns-read-zone -p -s -S unixtime > $@

out/root.hint: out/root.zone.signed
	grep -P '^(.*root\.ns\.xvnet0\.eu\.org\.|\.)\t+.*$$' $< > $@

out/iana-orig.zone:
	$(curl) https://www.internic.net/domain/root.zone

out/iana-strip.zone: out/iana-orig.zone iana-strip.txt 
	cat $< | perl -0p iana-strip.txt \
		| $(cleanzone) > $@

out/dn42-delegation-orig.zone:
	$(curl) 'https://explorer.burble.com/api/dns/root-zone?format=bind'

out/dn42-delegation.zone: gen-dn42-records.sh out/dn42-delegation-orig.zone
	./$< | $(cleanzone) > $@

out/opennic-glue-orig.zone:
	drill -t @168.119.153.26 AXFR opennic.glue. > $@

out/opennic-glue-strip.zone: out/opennic-glue-orig.zone
	cat $< | $(cleanzone) > $@

out/opennic-delegation.zone: gen-opennic-records.sh out/opennic-glue-strip.zone
	./$< | $(cleanzone) > $@

out/furnic.zone:
	drill -t @151.236.24.112 NS fur. > $@

out/dnssec-zsk.key out/dnssec-zsk.private:
	psk="$$(ldns-keygen -a ed25519 .)"; mv $$psk.private out/dnssec-zsk.private; mv $$psk.key out/dnssec-zsk.key

dnssec-ksk := $(wildcard .dnssec-ksk.private ../../../../../secrets/dns/root-ksk1.private)
out/root.zone.signed: out/root.zone out/dnssec-zsk.private $(dnssec-ksk)
	ldns-signzone -n -e "$$(date -u +%Y%m%d --date='+1month')" -f $@ -o . -z sha512 $< out/dnssec-zsk $(subst .private,,$(dnssec-ksk))
