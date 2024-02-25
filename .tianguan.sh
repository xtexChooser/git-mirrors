#!/usr/bin/env bash

tiang::target nl-alk1 ssh://nl-alk1.svr.xvnet.eu.org

tiangCommandsUsage+="""
    -ss --syncsec  [FILE]       Copy a secret file to targets
"""
tiang::defineCommand atremis::syncsec

atremis::syncsec() {
	if [[ "$1" == "-ss" || "$1" == "--syncsec" ]]; then
		[ $# -lt 2 ] && tiang::error "1 parameter is required for --syncsec"
		tiang::runParallelOnTargets tiang::runSCP "../secrets/$2" "/srv/secrets/$2"
		tiangHandledParams=2
	else
		tiangHandledParams=0
	fi
	exit 0
}
