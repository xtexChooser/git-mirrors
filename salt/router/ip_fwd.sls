# IPv4 rp filter
net.ipv4.conf.all.rp_filter:
    sysctl.present:
        - value: 0
net.ipv4.conf.default.rp_filter:
    sysctl.present:
        - value: 0
# https://github.com/systemd/systemd/blob/main/sysctl.d/50-default.conf#L26
net.ipv4.conf.*.rp_filter:
    sysctl.present:
        - value: 0

# IP forwarding
net.ipv4.conf.all.forwarding:
    sysctl.present:
        - value: 1
net.ipv6.conf.all.forwarding:
    sysctl.present:
        - value: 1

# firewalld IPv6 rp filter
/etc/firewalld/firewalld.conf:
    file.replace:
        pattern: ^(#\s*|)IPv6_rpfilter=.*$
        repl: IPv6_rpfilter=no
