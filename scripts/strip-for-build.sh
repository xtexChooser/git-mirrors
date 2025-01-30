#!/usr/bin/env bash

set -e

rm -vrf -- \
	tests extensions/*/tests skins/*/tests \
	**/.phan docs cache images .dockerignore .editorconfig .eslintignore \
	SECURITY UPGRADE INSTALL HISTORY \
	CREDITS COPYING RELEASE-NOTES-* **.md \
	mw-config \

echo "Done"
