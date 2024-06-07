#!/usr/bin/env bash
set -e

modlist="$1"
echo "" >"$modlist"

cat <<EOF
set root=(cd)/
set prefix=\$root/boot/grub
EOF

cat >>"$modlist" <<EOF
iso9660
biosdisk
normal
EOF

cat <<EOF
menuentry "Cane $CANE_VERSION" --id "cane" {
    multiboot \$root/boot/vinia/vinia-multiboot CANE_VERSION=$CANE_VERSION
    boot
}

set default=cane
set timeout=1
EOF
cat >>"$modlist" <<EOF
multiboot
EOF
