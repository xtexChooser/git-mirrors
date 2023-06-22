#! /usr/bin/bash
# shellcheck source=/dev/null
source .env
{
    printf "<details><summary>Credits</summary>\n\n"
    cat CREDITS.md
    echo "</details>"
    printf "\n"
    cat README.md
} | jo body=@/dev/stdin | curl -A "xtex-mp-pack scripts (HsMwyVxf)" \
    -H "Authorization: $MODRINTH_TOKEN" \
    -H "Content-Type: application/json" \
    -X PATCH -d "$(cat /dev/stdin)" https://api.modrinth.com/v2/project/HsMwyVxf && echo "Modrinth README updated"
