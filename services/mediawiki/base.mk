$(call load-state, services/caddy)

mediawiki-configs-dir := $(STATES_DIR)/services/mediawiki/config
mediawiki-configs := $(patsubst $(mediawiki-configs-dir)/%,%,$(wildcard $(mediawiki-configs-dir)/*))

$(call x-container-service)
V_SERVICE	= mediawiki
V_SVCDEPS	+= $(addprefix $(mediawiki-configs-dir)/,$(mediawiki-configs))
V_SVCDEPS	+= /var/run/mediawiki /var/lib/mediawiki
V_ARGS		+= --mount=type=bind,src=/srv/atremis/services/mediawiki/config,dst=/etc/mediawiki,ro=true
V_ARGS		+= --mount=type=bind,src=/srv/secrets/mw,dst=/srv/secrets/mw,ro=true
V_ARGS		+= --mount=type=bind,src=/var/run/mediawiki,dst=/var/run/mediawiki
V_ARGS		+= --mount=type=bind,src=/var/lib/mediawiki,dst=/var/lib/mediawiki
V_ARGS		+= --mount=type=image,source=codeberg.org/xvnet/x-mediawiki:latest,destination=/opt/mediawiki
V_ARGS		+= --label=org.eu.xvnet.x.depimgs=codeberg.org/xvnet/x-mediawiki:latest
V_ARGS		+= --memory=128M
V_ARGS 		+= codeberg.org/xvnet/x-mediawiki-php:latest
$(call end)

$(call podman-image)
V_NAME		= x-mediawiki
V_IMAGE		= codeberg.org/xvnet/x-mediawiki:latest
$(call end)

CADDY_INCLUDES += $(STATES_DIR)/services/mediawiki/Caddyfile

$(call add-fs-directory,/var/run/mediawiki)
$(call add-fs-directory,/var/lib/mediawiki)

$(call stamp)
V_NAME		= mediawiki-restart
V_DEPS		+= $(wildcard /srv/atremis/services/mediawiki/config/*)
V_POST		= dinit-restart E_SERVICE=mediawiki
$(call end)
