# shellcheck shell=bash

atre::atremis::update-containers() {
	local -a servicesToRestart

	updated=""
	for img in $(podman image ls --format "{{if .RepoTags}}{{index .RepoTags 0}}{{end}}"); do
		oldDigest="$(podman image ls "$img" --format "{{.Digest}}")"
		echo "== Pulling $img"
		podman pull --quiet "$img" >/dev/null || continue
		newDigest="$(podman image ls "$img" --format "{{.Digest}}")"

		if [[ "$oldDigest" != "$newDigest" ]]; then
			echo "=== Updated from $oldDigest to $newDigest"
			atre::publog "Updated OCI image: $img" \
				"OCI-Image: $img" \
				"OCI-From: $oldDigest" \
				"OCI-New: $newDigest"
			updated=true
			for svc in $(podman container ls --format "{{json .}}" | jq -s -r ".[] | select((.Labels[\"org.eu.xvnet.x.depimgs\"]? // \"\" | split(\",\") | contains([\"$img\"])) or (.Image == \"$img\")) | .Labels[\"org.eu.xvnet.x.dinitservice\"]? // \"\""); do
				[[ "$svc" == "" ]] && continue
				servicesToRestart=("${servicesToRestart[@]}" "$svc")
			done
		fi
	done

	if [[ -n "$updated" ]]; then
		while IFS= read -r -d '' svc; do
			echo "== Restarting service: $svc"
			atre::publog "$svc: Restarting due to OCI image update"
			dinitctl restart "$svc"
		done < <(printf "%s\0" "${servicesToRestart[@]}" | sort -uz)

		podman image prune -f
	fi
}
