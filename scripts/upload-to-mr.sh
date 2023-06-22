#! /usr/bin/bash
# shellcheck source=/dev/null
source .env

version=$(grep '^version = "' pack.toml | sed -e 's/version = "//' | sed -e 's/"//')

payload=$(jo -p project_id=HsMwyVxf \
    'file_parts=["file"]' featured=true 'loaders=["quilt"]' \
    version_type=release \
    "game_versions=[\"$(grep '^minecraft = "' pack.toml | sed -e 's/minecraft = "//' | sed -e 's/"//')\"]" \
    dependencies=[] \
    "version_number=$version" "name=$version")

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
