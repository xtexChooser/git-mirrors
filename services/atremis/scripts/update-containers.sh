#!/usr/bin/env bash
set -e

declare -a servicesToRestart

updated=""
for img in $(podman image ls --format "{{if .RepoTags}}{{index .RepoTags 0}}{{end}}"); do
	oldDigest="$(podman image ls "$img" --format "{{.Digest}}")"
	podman pull --quiet "$img" >/dev/null || continue
	newDigest="$(podman image ls "$img" --format "{{.Digest}}")"

	if [[ "$oldDigest" != "$newDigest" ]]; then
		echo "Updated image $img from $oldDigest to $newDigest"
		updated=true
		for svc in $(podman container ls --format "{{json .}}" | jq -s -r ".[] | select((.Labels[\"org.eu.xvnet.x.depimgs\"]? // \"\" | split(\",\") | contains([\"$img\"])) or (.Image == \"$img\")) | .Labels[\"org.eu.xvnet.x.dinitservice\"]? // \"\""); do
			[[ "$svc" == "" ]] && continue
			servicesToRestart=("${servicesToRestart[@]}" "$svc")
		done
	fi
done

if [[ -n "$updated" ]]; then
	while IFS= read -r -d '' svc; do
		echo "Restarting container service $svc"
		dinitctl restart "$svc"
	done < <(printf "%s\0" "${servicesToRestart[@]}" | sort -uz)

	atre apply
	podman image prune -f
fi
