var REG_NONE = NewRegistrar("none");
var DNS_NONE = NewDnsProvider("none");
var DNS_BIND = NewDnsProvider("bind");
var DNS_CLOUDFLARE = NewDnsProvider("cloudflare");

DEFAULTS(TTL(300), CF_PROXY_DEFAULT_OFF, DnsProvider(DNS_BIND));

require_glob("./zones/");
require_glob("./servers/");
require_glob("./services/");
