$(call x-container-service)
V_SERVICE	= monto
V_DEPS		+= /var/lib/mariadb/monto/my.cnf
V_SVCDEPS	+= /var/lib/mariadb/monto /var/run/mariadb
V_ARGS		+= --mount=type=bind,src=/var/lib/mariadb/monto,dst=/var/lib/mariadb
V_ARGS		+= --mount=type=bind,src=/var/run/mariadb,dst=/var/run/mariadb
V_ARGS		+= --memory=128M
V_ARGS		+= --publish=5433:5432/tcp
V_ARGS 		+= codeberg.org/xvnet/mariadb:latest
$(call end)

$(call fs-file)
V_PATH		= /var/lib/mariadb/monto/my.cnf
V_TEMPLATE	= bash-tpl $(STATES_DIR)/services/mariadb/monto/my.cnf
$(call end)

$(call add-fs-directory,/var/lib/mariadb/monto)
$(call add-fs-directory,/var/run/mariadb)
