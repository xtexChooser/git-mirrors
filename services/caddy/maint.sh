# shellcheck shell=bash

atre::caddy::reload() {
	podman container exists caddy || return

	echo 'Reloading Caddy ...'
	podman exec -it caddy caddy reload --config /etc/caddy/Caddyfile
	echo 'Reloaded Caddy'
	return
}

atre::caddy::validate() {
	make HOSTNAME=opilio.s.xvnet0.eu.org USER=root LEONIS_LOAD_ALL=y do-tpl TPL_BACKEND=bash-tpl \
		TPL_IN=services/caddy/Caddyfile TPL_OUT=.caddy-validation.Caddyfile
	sed -i -e 's/include/#include/' .caddy-validation.Caddyfile

	podman run -it --rm --name caddy-validate -v "$(pwd)":/validate \
		codeberg.org/xens/x-caddy:latest \
		caddy validate --config /validate/.caddy-validation.Caddyfile --adapter caddyfile

	rm -f .caddy-validation.Caddyfile
}
