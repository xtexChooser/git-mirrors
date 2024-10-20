# shellcheck shell=bash

atre::bird::c() {
	podman exec -it bird birdc "$@"
	return
}

atre::bird::stop() {
	podman container exists bird || return
	atre::bird::c graceful restart
	return
}

atre::bird::restart() {
	podman container exists bird || return
	atre::bird::c graceful restart
	dinitctl restart bird
	return
}

atre::bird::reconf() {
	podman container exists bird || return

	podman exec -it bird bird -p || {
		echo "BIRD configuration validation failed" >&2
		exit 1
	}

	echo 'Reconfiguring BIRD...'
	atre::bird::c configure
	echo 'Reconfigured BIRD'

	return
}

atre::bird::validate() {
	make HOSTNAME=opilio.s.xvnet0.eu.org USER=root LEONIS_LOAD_ALL=y do-tpl TPL_BACKEND=bash-tpl \
		TPL_IN=services/bird/conf/bird.conf TPL_OUT=.bird-valid.conf
	sed -i -e 's/include/#include/' .bird-valid.conf

	podman run -it --rm --name bird-validate -v "$(pwd)":/validate \
		--privileged \
		codeberg.org/xens/bird:latest \
		-p -c /validate/.bird-valid.conf

	rm -f .bird-valid.conf
}

atre::bird::update-dn42-roa() {
	curl -SL -o /var/cache/bird/dn42_roa_v4.conf https://explorer.burble.com/api/roa/bird/2/4
	curl -SL -o /var/cache/bird/dn42_roa_v6.conf https://explorer.burble.com/api/roa/bird/2/6
	atre::bird::reconf
	return
}
