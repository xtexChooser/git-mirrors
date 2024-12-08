deploy:
    rm -rf public; hugo build --enableGitInfo --minify
    rsync -r --force --delete-after public/ p.projectsegfau.lt:apps/blog
