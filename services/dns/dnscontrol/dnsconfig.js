var REG_NONE = NewRegistrar("none");

var DSP_BIND = NewDnsProvider("bind");
var DSP_DESEC = NewDnsProvider("desec");
var DSP_GCORE = NewDnsProvider("gcore");

require("converted/xvnet0.eu.org.js");
D_EXTEND("xvnet0.eu.org", DnsProvider(DSP_DESEC, 0), DnsProvider(DSP_GCORE, 0));

require("converted/xvnet.eu.org.js");
D_EXTEND("xvnet.eu.org", DnsProvider(DSP_DESEC, 0), DnsProvider(DSP_GCORE, 0));

var domains = getConfiguredDomains();
for (i = 0; i < domains.length; i++)
	D_EXTEND(domains[i], DnsProvider(DSP_BIND));
