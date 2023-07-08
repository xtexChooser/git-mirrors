$(call systemd-unit)
V_UNIT		= caddy.service
V_ENABLED	= true
V_RUNNING	= true
$(call end)

$(call systemd-unit)
V_UNIT		= nginx.service
V_ENABLED	= true
V_RUNNING	= true
$(call end)
