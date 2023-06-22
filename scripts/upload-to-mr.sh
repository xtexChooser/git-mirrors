#! /usr/bin/bash
# shellcheck source=/dev/null
source .env

version=$(grep '^version = "' pack.toml | sed -e 's/version = "//' | sed -e 's/"//')
echo "$version"

payload=$(jo -- project_id=HsMwyVxf \
    file_parts="$(jo -a file)" featured=false loaders="$(jo -a quilt)" \
    version_type=release \
    game_versions="$(jo -a "$(grep '^minecraft = "' pack.toml | sed -e 's/minecraft = "//' | sed -e 's/"//')")" \
    dependencies="$(jo -a < /dev/null)" \
    -s "version_number=$version" -s "name=$version" \
    "changelog=$(echo -e "${changelog:-unavailable}")")

echo "$payload"

rm -- *.mrpack
packwiz modrinth export
file=$(echo *.mrpack)
echo "$file"

curl -A "xtex-mp-pack scripts (HsMwyVxf)" \
    -H "Authorization: $MODRINTH_TOKEN" \
    -H "Content-Type: multipart/form-data" \
    -X POST -F data="$payload" -F file="@$file" https://api.modrinth.com/v2/version
printf "\n"

echo Modrinth version uploaded
