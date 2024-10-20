DN42_LOCAL_IP := 172.20.206.$(shell echo "$$((64 + $(XVNET_NUM)))")
BIRD_ROUTER_ID ?= $(DN42_LOCAL_IP)
XVNET_ASN := $(shell echo "$$((4244310000 + $(XVNET_NUM)))")
XVNET_LOCAL_IP := fd00:443a:ef14:1::$(shell printf '%x:%x' \
	$$(($(XVNET_NUM) / 0x10000)) $$(($(XVNET_NUM) % 0x10000)))

BIRD_VERSION ?= 2.15.1

$(call x-container-service)
V_SERVICE	= bird
V_DEPS_ORD	+= /etc/bird/bird.conf /var/run/bird
V_PRE_STOP	= $(abspath $(STATES_DIR)/atre) svc bird stop
V_ARGS		+= --mount=type=bind,src=/etc/bird,dst=/etc/bird,ro=true
V_ARGS		+= --mount=type=bind,src=/var/cache/bird,dst=/var/cache/bird
V_ARGS		+= --mount=type=bind,src=/var/run/bird,dst=/var/run/bird
V_ARGS		+= --publish-all
V_ARGS		+= --network=host --ipc=host --cap-add=CAP_NET_ADMIN --privileged
V_ARGS 		+= codeberg.org/xens/bird:$(strip $(BIRD_VERSION))
V_ARGS 		+= -R
$(call end)

$(call add-fs-directory,/var/cache/bird)
$(call add-fs-directory,/var/run/bird)

$(call cmd-stamp)
V_NAME		= bird-conf
V_CMD		= $(STATES_DIR)/atre svc bird reconf
V_DEPS		+= /etc/bird/bird.conf
V_DEPS_ORD	+= dinit-bird
$(call end)

$(call fs-file)
V_PATH		= /etc/bird/bird.conf
V_TEMPLATE	= bash-tpl $(STATES_DIR)/services/bird/conf/bird.conf
V_DEP_VARS	+= BIRD_ROUTER_ID XVNET_NUM DN42_LOCAL_IP XVNET_ASN XVNET_LOCAL_IP
V_DEPS		+= $(wildcard $(STATES_DIR)/services/bird/conf/*.conf)
$(call end)

$(call fs-file)
V_PATH		= /etc/cron.d/dn42-roa
V_TEMPLATE	= bash-tpl $(STATES_DIR)/services/bird/cron-dn42-roa
V_DEP_VARS	+= STATES_DIR
V_POST		+= systemd-restart E_UNIT=cronie.service
V_DEPS		= pkg-cronie
$(call end)

$(call fs-file)
V_PATH		= /etc/sysctl.d/95-ip-fwd.conf
V_TEMPLATE	= bash-tpl $(STATES_DIR)/services/bird/sysctl
V_DEP_VARS	+= STATES_DIR
V_POST		+= sysctl-reload
$(call end)
