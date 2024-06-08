#!/usr/bin/env bash
set -e

modlist="$1"
echo "" >"$modlist"

cat <<EOF
insmod iso9660
insmod biosdisk
set root=(cd)
set prefix=(\$root)/boot/grub
EOF

cat >>"$modlist" <<EOF
iso9660
biosdisk
normal
EOF

cat <<EOF
menuentry "Cane $CANE_VERSION" --id "cane" {
    insmod multiboot
    multiboot (\$root)/boot/vinia/multiboot cane.version=$CANE_VERSION
	module (\$root)/boot/vinia/vinia vinia.multiboot.core cane.version=$CANE_VERSION
    boot
}

set default=cane
set timeout=1
EOF
cat >>"$modlist" <<EOF
multiboot
EOF
