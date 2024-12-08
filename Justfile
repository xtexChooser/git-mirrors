deploy:
    rm -rf public; hugo build --enableGitInfo --minify
    rsync -vrc --force --delete-after public/ p.projectsegfau.lt:apps/blog
