#!/usr/bin/env bash
set -e

while [[ "$#" -gt 0 ]]; do
	eval "$1"
	shift
done

# Setup xorriso
cat <<EOF
-overwrite on
-disk_pattern on
-pathspecs on
-iso_rr_pattern on
# -pkt_output on
-uid 0
-gid 0
EOF

# ISO info
cat <<EOF
-volid CANE
-publisher CANE
-application_id $CANE_VERSION
-system_id CANE
# -copyright_file
EOF

# Boot
cat <<EOF
-add /boot/grub/grub.cfg="$GRUB_CFG" --

-boot_image grub discard
-add /boot/grub/eltorito.img="$GRUB_ELTORITO" --
-boot_image grub id_string=CANE_GRUB
-boot_image grub grub2_boot_info=on
-boot_image grub emul_type=no_emulation
-boot_image grub load_size=2048
-boot_image grub bin_path=/boot/grub/eltorito.img
-boot_image grub partition_table=on
-boot_image grub next
EOF

# Vinia
cat <<EOF
-add \
    /boot/vinia/multiboot="$VINIA_MULTIBOOT" \
    --
EOF

# End
cat <<EOF
EOF
