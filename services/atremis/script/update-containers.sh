#!/usr/bin/env bash
set -e

updated=""
for img in $(podman image ls --format "{{if .RepoTags}}{{index .RepoTags 0}}{{end}}"); do
	oldDigest="$(podman image ls "$img" --format "{{.Digest}}")"
	podman pull "$img" || continue
	newDigest="$(podman image ls "$img" --format "{{.Digest}}")"

	if [[ "$oldDigest" != "$newDigest" ]]; then
		echo Updated container image "$img"
		updated=true
		for svc in $(podman container ls --format "{{if eq .Image \"$img\"}}{{index .Labels \"org.eu.xvnet.x.dinitservice\"}}{{end}}"); do
			[[ "$svc" == "" ]] && continue
			echo Restarting container service "$svc"
			dinitctl restart "$svc"
		done
	fi
done

if [[ -n "$updated" ]]; then
	atre pull
	podman image prune -f
	atre apply
fi
