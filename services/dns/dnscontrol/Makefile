.DELETE_ON_ERROR:

$(shell mkdir -p out)

out/dnsconfig.js: dnsconfig.js out/creds.json \
	out/converted/xvnet0.eu.org.js \
	out/converted/xvnet.eu.org.js
	cp $< $@

out/creds.json: creds.json
	cp $< $@

out/converted/xvnet0.eu.org.js: ../zones/xvnet0.eu.org.zone scripts/convert.sh
	scripts/convert.sh xvnet0.eu.org

out/converted/xvnet.eu.org.js: ../zones/xvnet.eu.org.zone scripts/convert.sh
	scripts/convert.sh xvnet.eu.org
