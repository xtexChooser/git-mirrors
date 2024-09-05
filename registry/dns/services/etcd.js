D_EXTEND(
    "etcd.infra.xvnet.eu.org",
    SRV("_etcd-server-ssl._tcp", 10, 10, 2380, "nl-alk1.svr.xvnet.eu.org."),
    SRV("_etcd-server-ssl._tcp", 10, 10, 2380, "us-dal1.svr.xvnet.eu.org."),
    CNAME("_etcd-client-ssl._tcp", "_etcd-server-ssl._tcp.xvnet.eu.org.")
);
