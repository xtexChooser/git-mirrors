var REG_NONE = NewRegistrar("none");

var DSP_BIND = NewDnsProvider("bind");
var DSP_DESEC = NewDnsProvider("desec");
var DSP_GCORE = NewDnsProvider("gcore");

require("converted/xvnet0.eu.org.js");
D_EXTEND("xvnet0.eu.org", DnsProvider(DSP_DESEC, 0), DnsProvider(DSP_GCORE, 0));

require("converted/xvnet.eu.org.js");
D_EXTEND("xvnet.eu.org", DnsProvider(DSP_GCORE, 0));

require("converted/0.c.6.1.7.0.1.b.e.0.a.2.ip6.arpa.js");
// UPSTREAM: https://github.com/StackExchange/dnscontrol/issues/2889
// D_EXTEND("0.c.6.1.7.0.1.b.e.0.a.2.ip6.arpa", DnsProvider(DSP_GCORE, 0));

// var domains = getConfiguredDomains();
// for (i = 0; i < domains.length; i++)
// 	D_EXTEND(domains[i], DnsProvider(DSP_BIND));
