define systemd-unit
$(eval $(call systemd-unit0))
endef

define systemd-unit0
# ensure enabled
$(if $(SD_ENABLED),
$(if $(findstring true,$(SD_ENABLED)),$(call add-state-ninja,build tests/systemd/$(SD_UNIT)-enabled: systemd-enable\n\tSD_UNIT=$(SD_UNIT)),
$(if $(findstring false,$(SD_ENABLED)),$(call add-state-ninja,build tests/systemd/$(SD_UNIT)-disabled: systemd-disable\n\tSD_UNIT=$(SD_UNIT)),
$(error SD_ENABLED $(SD_ENABLED) is invalid)
)))

# ensure started
$(if $(SD_STARTED),
$(if $(findstring true,$(SD_STARTED)),$(call add-state-ninja,build tests/systemd/$(SD_UNIT)-started: systemd-start\n\tSD_UNIT=$(SD_UNIT)),
$(if $(findstring false,$(SD_STARTED)),$(call add-state-ninja,build tests/systemd/$(SD_UNIT)-stopped: systemd-stop\n\tSD_UNIT=$(SD_UNIT)),
$(error SD_STARTED $(SD_STARTED) is invalid)
)))

SD_ENABLED = $(empty)
SD_STARTED = $(empty)
SD_UNIT = $(empty)
endef
