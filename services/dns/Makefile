.DELETE_ON_ERROR:

$(shell mkdir -p out out/zones out/zsk)

strip-zone=$(patsubst zones/%,%,$(patsubst %.zone,%,$1))
zones:=$(foreach file,$(wildcard zones/*.zone),$(call strip-zone,$(file)))

all: $(addprefix out/zones/,$(addsuffix .zone,$(zones)))

dnssec-ksk := $(wildcard .dnssec-ksk.private ../../../../../secrets/dns/zone-ksk1.private)
out/zsk/%.key out/zsk/%.private:
	psk="$$(ldns-keygen -a ed25519 $*.)"; mv $$psk.private out/zsk/$*.private; mv $$psk.key out/zsk/$*.key
.NOTINTERMEDIATE: $(patsubst %,out/zsk/%.key out/zsk/%.private,$(zones))

out/zones/%.zone: zones/%.zone out/zsk/%.private
	ldns-read-zone -p -S unixtime $< \
		| ldns-signzone -e "$$(date -u +%Y%m%d --date='+14days')" -f /dev/stdout -o $*. -z sha512 /dev/stdin out/zsk/$* $(subst .private,,$(dnssec-ksk)) \
		> $@
