$(call x-container-service)
V_SERVICE	= bird
V_SVCDEPS	+= /etc/bird/bird.conf /var/run/bird
V_PRE_STOP	= $(abspath $(STATES_DIR)/services/bird/stop.sh)
V_ARGS		+= --mount=type=bind,src=/etc/bird,dst=/etc/bird,ro=true
V_ARGS		+= --mount=type=bind,src=/var/run/bird,dst=/var/run/bird
V_ARGS		+= --publish-all
V_ARGS		+= --network=host --ipc=host --cap-add=CAP_NET_ADMIN
V_ARGS 		+= codeberg.org/xvnet/bird:2.14
V_ARGS 		+= -R
$(call end)

$(call add-fs-directory,/var/run/bird)

$(call cmd-stamp)
V_NAME		= bird-conf
V_CMD		= $(STATES_DIR)/services/bird/reconf.sh
V_DEPS		+= /etc/bird/bird.conf
$(call end)

BIRD_INCLUDES :=
$(call fs-file)
V_PATH		= /etc/bird/bird.conf
V_TEMPLATE	= bash-tpl $(STATES_DIR)/services/bird/conf/bird.conf
V_DEP_VARS	+= BIRD_INCLUDES
V_DEP_VARS	+= BIRD_ROUTER_ID DN42_LOCAL_IP XVNET_ASN XVNET_LOCAL_IP
$(call end)
