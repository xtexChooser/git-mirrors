LEONIS_BASE_DIR = ./external/leonis
STATES_DIR = .
ATRE_DIR = /srv/atremis

define vendor-targets
$(call load-state, services/atremis)
$(eval include hosts/$(HOSTNAME).mk)
$(eval -include /srv/secrets/atre/$(HOSTNAME).mk)
endef

include $(LEONIS_BASE_DIR)/Makefile
