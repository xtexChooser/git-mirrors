#!/usr/bin/env bash
set -e
podman container exists bird || exit
podman exec -it bird birdc graceful restart
dinitctl restart bird
