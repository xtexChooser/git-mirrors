update-fs:
    salt.runner:
        - name: fileserver.update

update-git-pillar:
    salt.runner:
        - name: git_pillar.update

update-pillar:
    salt.function:
        - name: saltutil.refresh_pillar
        - tgt: "*"
        - require:
            - salt: update-git-pillar

#apply-highstate:
#    salt.function:
#        - name: state.apply
#        - tgt: "*"
#        - require:
#            - salt: update-fs
#            - salt: update-pillar
