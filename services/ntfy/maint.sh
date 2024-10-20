# shellcheck shell=bash

atre::ntfy::cli() {
	podman exec -it ntfy ntfy "$@"
	return
}
