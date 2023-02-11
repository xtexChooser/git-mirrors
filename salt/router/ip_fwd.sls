# IPv4 rp filter
net.ipv4.conf.all.rp_filter:
    sysctl.present:
        - value: 0
net.ipv4.conf.default.rp_filter:
    sysctl.present:
        - value: 0
# https://github.com/systemd/systemd/blob/main/sysctl.d/50-default.conf#L26
/lib/sysctl.d/50-default.conf:
    file.replace:
        pattern: ^net\.ipv4\.conf\.\*\.rp_filter\s*=\s*2$
        repl: "# net.ipv4.conf.*.rp_filter = 2"
        ignore_if_missing: True

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
        append_if_not_found: True
