$(call x-container-service)
V_SERVICE	= bind
V_DEPS		+= /etc/bind/named.conf
V_SVCDEPS	+= /var/run/bind podman-image-dns-root-zone
V_ARGS		+= --mount=type=bind,src=/etc/bind,dst=/etc/bind,ro=true
V_ARGS		+= --mount=type=bind,src=/var/run/bind,dst=/var/run/bind
V_ARGS		+= --mount=type=image,source=codeberg.org/xvnet/dns-root-zone:latest,destination=/opt/root-zone
V_ARGS		+= --publish-all
V_ARGS		+= --network=host --cap-add=CAP_NET_BIND_SERVICE
V_ARGS 		+= codeberg.org/xvnet/bind:latest
$(call end)

$(call add-fs-directory,/var/run/bind)

$(call fs-file)
V_PATH		= /etc/bind/named.conf
V_TEMPLATE	= bash-tpl $(STATES_DIR)/services/bind/conf/named.conf
V_DEPS		+= $(wildcard $(STATES_DIR)/services/bind/conf/*.conf)
$(call end)

$(call podman-image)
V_NAME		= dns-root-zone
V_IMAGE		= codeberg.org/xvnet/dns-root-zone:latest
$(call end)
