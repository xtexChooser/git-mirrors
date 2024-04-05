#!/usr/bin/env bash

set -e

defaultBranch="$(yq ". | filter(.name == \"core\") | .[0].defaultBranch // .[0].branch" repositories.yaml)"
echo "Default branch: $defaultBranch"

while read -r repo; do
	name="$(yq '.name' <<<"$repo")"
	subtreePrefix="$(yq '.prefix' <<<"$repo")"
	giturl="$(yq '.git' <<<"$repo")"
	branch="$(yq ".branch // \"$defaultBranch\"" <<<"$repo")"

	if [[ -e "$subtreePrefix" ]]; then
		echo "Merging repository: $name"
		if ! git subtree -P "$subtreePrefix" pull "$giturl" "$branch"; then
			echo "Trying to resolve core merge conflicts"
			grep 'path = ' "$subtreePrefix"/.gitmodules | cut -d'=' -f2 | awk '{print "'"$subtreePrefix"'/" $1}' | while read -r dir; do
				{
					rm -d "$dir"
					git add "$dir"
					git rm --cached "$dir~*"
					rm -d "$dir~*"
				} &>/dev/null || true
			done
			git merge --continue
		fi
	else
		echo "Initializing repository: $name"
		git subtree -P "$subtreePrefix" add "$giturl" "$branch"
	fi

	grep 'path = ' "$subtreePrefix"/.gitmodules | cut -d'=' -f2 | awk '{print "'"$subtreePrefix"'/" $1}' | while read -r dir; do
		rm -d "$dir" &>/dev/null || true
	done
	if ! git diff-index --quiet HEAD --; then
		git add "$subtreePrefix"
		git commit -m "Remove submodule in $subtreePrefix"
	fi
done <<<"$(yq -o=json -I=0 '. | sort_by(.pri // 100) | .[]' repositories.yaml)"

echo "Done"
