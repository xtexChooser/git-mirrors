#!/usr/bin/env bash
set -e

outfile="$1"
modlist="$2"

# shellcheck disable=SC2046
exec grub2-mkimage --format i386-pc-eltorito -p /boot/grub -o "$outfile" $(cat "$modlist")
