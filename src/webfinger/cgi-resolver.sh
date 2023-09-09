#!/bin/bash

#resource=$1
file=${PATH_INFO/%.json/.txt}
file=${file#\/}
file=${file// /+}

if [[ -f "$file" ]]; then
	target=$(< "$file")
	printf "Status: 307 Temporary Redirect\n"
	printf "Content-type: text/plain\n"
	printf "Location: %s\n" "$target"
	printf "\n"

	printf "Resolved redirect to [%s]\n" "$target"
else
	printf "Status: 404 Not Found\n"
	printf "X-Resolver: xtex-home/webfinger/cgi-resolver.sh\n"
	printf "X-Resolaver: %s\n" "$file"
	printf "\n"
fi

exit 0
