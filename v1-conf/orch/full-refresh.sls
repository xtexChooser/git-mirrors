clear-all:
    salt.function:
        - name: saltutil.clear_cache
        - tgt: "*"

clear-fs:
    salt.runner:
        - name: fileserver.clear_cache

update-fs:
    salt.runner:
        - name: fileserver.update
        - require:
            - salt: clear-fs

clear-pillar:
    salt.runner:
        - name: pillar.clear_pillar_cache

update-git-pillar:
    salt.runner:
        - name: git_pillar.update
        - require:
            - salt: clear-pillar

update-pillar:
    salt.function:
        - name: saltutil.refresh_pillar
        - tgt: "*"
        - require:
            - salt: clear-all
            - salt: update-git-pillar

sync-all:
    salt.runner:
        - name: saltutil.sync_all
        - require:
            - salt: clear-all
            - salt: update-pillar
            - salt: update-fs

apply-highstate:
    salt.function:
        - name: state.apply
        - tgt: "*"
        - require:
            - salt: sync-all
