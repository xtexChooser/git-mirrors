.DELETE_ON_ERROR:

$(shell mkdir -p out)

out/dnsconfig.js: dnsconfig.js out/creds.json \
	out/converted/xvnet0.eu.org.js \
	out/converted/xvnet.eu.org.js \
	out/converted/0.c.6.1.7.0.1.b.e.0.a.2.ip6.arpa.js
	cp $< $@

out/creds.json: creds.json
	cp $< $@

out/converted/xvnet0.eu.org.js: ../zones/xvnet0.eu.org.zone scripts/convert.sh
	scripts/convert.sh xvnet0.eu.org

out/converted/xvnet.eu.org.js: ../zones/xvnet.eu.org.zone scripts/convert.sh
	scripts/convert.sh xvnet.eu.org

out/converted/0.c.6.1.7.0.1.b.e.0.a.2.ip6.arpa.js: ../zones/0.c.6.1.7.0.1.b.e.0.a.2.ip6.arpa.zone scripts/convert.sh
	scripts/convert.sh 0.c.6.1.7.0.1.b.e.0.a.2.ip6.arpa
