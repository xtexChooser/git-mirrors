/srv/mw:
    file.directory:
        - user: root
        - group: root
        - dir_mode: 664

/srv/mw/src:
    file.directory:
        - require:
            - file: /srv/mw
    git.cloned:
        - name: https://github.com/wikimedia/mediawiki.git
        - target: /srv/mw/src
        - require:
            - file: /srv/mw/src

#mediawiki-src: