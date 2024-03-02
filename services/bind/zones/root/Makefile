.DELETE_ON_ERROR:
SHELL:=bash -c -e

curl = curl -q -o $@
cleanzone = ldns-read-zone -e RRSIG -e ZONEMD -e NSEC -e SOA

out/root.zone: root.zone \
	out/iana-strip.zone \
	out/dn42-delegation.zone \
	out/opennic-glue-strip.zone
	cat $^ | ldns-read-zone -p -s -S "$$(date -u +%Y%m%d)" > $@

out/iana-orig.zone:
	$(curl) https://www.internic.net/domain/root.zone

out/iana-strip.zone: out/iana-orig.zone iana-strip.txt 
	sed -E -f iana-strip.txt $< \
		| $(cleanzone) > $@

out/dn42-delegation.json:
	$(curl) https://explorer.burble.com/api/registry/domain/delegation-servers.dn42

out/dn42-delegation.zone: gen-dn42-records.sh out/dn42-delegation.json
	./$< | $(cleanzone) > $@

out/opennic-glue-orig.zone:
	drill -t @168.119.153.26 AXFR opennic.glue. > $@

out/opennic-glue-strip.zone: out/opennic-glue-orig.zone
	cat $< | $(cleanzone) > $@

out/opennic-delegation.zone: gen-opennic-records.sh out/opennic-glue-strip.zone
	./$< | $(cleanzone) > $@
