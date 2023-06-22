#! /usr/bin/bash
PACKWIZ=${PACKWIZ:-packwiz}

$PACKWIZ list | sort | uniq | awk '{ print "- " $0 }' | node -e "const fs = require('fs'); fs.writeFileSync('CREDITS.md', fs.readFileSync('CREDITS.md').toString().replace(/<!--BEGIN MOD LIST-->((.|\\n)*)<\!--END MOD LIST-->/m, '<!--BEGIN MOD LIST-->\\n\\n' + fs.readFileSync('/dev/stdin') + '\\n<\!--END MOD LIST-->'))"

git log --pretty='%aN' | sort | uniq | awk '{ print "- " $0 }' | node -e "const fs = require('fs'); fs.writeFileSync('CREDITS.md', fs.readFileSync('CREDITS.md').toString().replace(/<!--BEGIN CONTRIBUTORS LIST-->((.|\\n)*)<\!--END CONTRIBUTORS LIST-->/m, '<!--BEGIN CONTRIBUTORS LIST-->\\n\\n' + fs.readFileSync('/dev/stdin') + '\\n<\!--END CONTRIBUTORS LIST-->'))"

grep '^mod-id = ".*"$' --include='*.pw.toml' -r . --with-filename --only-matching | sed -e 's/:.*//g' | while read -r file
do
name=$(grep '^name = ".*"$' "$file" --no-filename | sed -e 's/name = "//' | sed -e 's/"//')
modid=$(grep '^mod-id = ".*"$' "$file" --no-filename | sed -e 's/mod-id = "//' | sed -e 's/"//')
printf "[%s](%s)\n" "$name" "https://modrinth.com/mod/$modid" | awk '{ print "- " $0 }'
done | node -e "const fs = require('fs'); fs.writeFileSync('CREDITS.md', fs.readFileSync('CREDITS.md').toString().replace(/<!--BEGIN MR LINKS LIST-->((.|\\n)*)<\!--END MR LINKS LIST-->/m, '<!--BEGIN MR LINKS LIST-->\\n\\n' + fs.readFileSync('/dev/stdin') + '\\n<\!--END MR LINKS LIST-->'))"

$PACKWIZ refresh
