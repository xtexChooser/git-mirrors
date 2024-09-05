$(call x-container-service)
V_SERVICE	= bind
V_DEPS		+= /etc/bind/named.conf
V_DEPS_ORD	+= /var/run/bind podman-image-dns-root-zone podman-image-dns-zones
V_ARGS		+= --mount=type=bind,src=/etc/bind,dst=/etc/bind,ro=true
V_ARGS		+= --mount=type=bind,src=/var/run/bind,dst=/var/run/bind
V_ARGS		+= --mount=type=image,source=codeberg.org/xens/dns-root-zone:latest,destination=/opt/root-zone
V_ARGS		+= --mount=type=image,source=codeberg.org/xens/dns-zones:latest,destination=/opt/zones
V_ARGS		+= --label=org.eu.xvnet.x.depimgs=codeberg.org/xens/dns-root-zone:latest,codeberg.org/xens/dns-zones:latest
V_ARGS		+= --publish-all
V_ARGS		+= --network=host --cap-add=CAP_NET_BIND_SERVICE
V_ARGS		+= --memory=128M
V_ARGS 		+= codeberg.org/xens/bind:latest
$(call end)

$(call add-fs-directory,/var/run/bind)

$(call fs-file)
V_PATH		= /etc/bind/named.conf
V_TEMPLATE	= bash-tpl $(STATES_DIR)/services/bind/conf/named.conf
V_DEPS		+= $(wildcard $(STATES_DIR)/services/bind/conf/*.conf)
$(call end)

$(call podman-image)
V_NAME		= dns-root-zone
V_IMAGE		= codeberg.org/xens/dns-root-zone:latest
$(call end)

$(call podman-image)
V_NAME		= dns-zones
V_IMAGE		= codeberg.org/xens/dns-zones:latest
$(call end)

$(call fs-line)
V_NAME		= recursive-ns
V_PATH		= /etc/resolv.conf
V_PREPEND	= true
V_LINE		= nameserver fd00:443a:ef14:2::2
V_MATCH		= ^nameserver\s+fd00:443a:ef14:2::2$$
$(call end)
