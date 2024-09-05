D_EXTEND(
    "xvnet.eu.org",
    MX("@", 100, "route3.mx.cloudflare.net."),
    MX("@", 27, "route2.mx.cloudflare.net."),
    MX("@", 21, "route1.mx.cloudflare.net."),
    DMARC_BUILDER({
        policy: "quarantine",
        ruf: ["mailto:abuse@xvnet.eu.org", "mailto:xtexChooser@duck.com"],
    }),
    TXT("@", "v=spf1 include:_spf.mx.cloudflare.net ~all")
);
