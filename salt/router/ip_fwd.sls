# IPv4 rp filter
net.ipv4.conf.all.rp_filter:
    sysctl.present:
        - value: 0
net.ipv4.conf.default.rp_filter:
    sysctl.present:
        - value: 0

# IP forwarding
net.ipv4.conf.all.forwarding:
    sysctl.present:
        - value: 1
net.ipv6.conf.all.forwarding:
    sysctl.present:
        - value: 1
