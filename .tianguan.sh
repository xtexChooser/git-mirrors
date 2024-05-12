#!/usr/bin/env bash

while read -r line; do
	# shellcheck disable=SC2046
	eval $(tail -c+5 <<<"$line")
done <<<"$(grep -E '^### tiang::' hosts/*.mk)"

tiang::defineCommand atremis::syncsec
tiangCommandsUsage+="""
    -ss --syncsec  [FILE]       Copy a secret file to targets
"""
atremis::syncsec() {
	if [[ "$1" == "-ss" || "$1" == "--syncsec" ]]; then
		[ $# -lt 2 ] && tiang::error "1 parameter is required for --syncsec"
		tiang::runParallelOnTargets tiang::runSCP "../secrets/$2" "/srv/secrets/$2"
		tiangHandledParams=2
	else
		tiangHandledParams=0
	fi
}
