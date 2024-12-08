version := 'git~' + shell("git describe --long --always | tr -d '\\n'")

default: build push deploy

build:
	podman build \
		--force-rm \
		--squash \
		--build-arg "VERSION={{version}}" \
		--tag codeberg.org/xtex/home \
		.

push:
	podman image push codeberg.org/xtex/home

deploy:
	ssh p.projectsegfau.lt -- '\
		podman image pull codeberg.org/xtex/home; \
		systemctl --user restart xtex-home; \
	'
