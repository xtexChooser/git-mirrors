#!/usr/bin/bash
set -euo pipefail

echo Updating MAS
curl -SL https://github.com/massgravel/Microsoft-Activation-Scripts/raw/refs/heads/master/MAS/All-In-One-Version-KL/MAS_AIO.cmd |
    sed -e 's/set old=1/set old=/g' | # remove update checker
    cat >MAS_AIO.cmd

echo Updating Cubic-11 license
curl -SL https://github.com/ACh-K/Cubic-11/raw/refs/heads/main/OFL.txt >Cubic_11-LICENSE.txt

echo Updating Cubic-11 TTF
curl -SL https://github.com/ACh-K/Cubic-11/raw/refs/heads/main/fonts/ttf/Cubic_11.ttf >Cubic_11.ttf
rm -f Cubic_11.ttf.xz
xz -e -9 Cubic_11.ttf
