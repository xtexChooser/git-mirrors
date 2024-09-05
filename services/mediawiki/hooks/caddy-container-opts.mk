V_ARGS		+= --mount=type=image,source=codeberg.org/xens/x-mediawiki:latest,destination=/opt/mediawiki
V_ARGS		+= --mount=type=bind,src=/var/lib/mediawiki,dst=/var/lib/mediawiki,ro=true
V_ARGS		+= --label=org.eu.xvnet.x.depimgs=codeberg.org/xens/x-mediawiki:latest
