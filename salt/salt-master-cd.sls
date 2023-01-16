salt-master-cd:
    user.present:
        - fullname: XTEX-VNET Salt Master CD Bot
        - shell: /bin/bash
        - home: /home/salt-master-cd
        - gid: root
        - groups:
            - sudo
        - password: '$5$Wkal.rtu2wnUByca$i1i7gwpuaGWYlnG5MfVRaBK1KudPC1FQzAzhPa0dqm3'
    ssh_auth.manage:
        - user: salt-master-cd
        - enc: ed25519
        - comment: salt-cd-user-infra@xvnet.eu.org
        - ssh_keys:
            - ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIFyJBe4KhpSI+WmiMgyqoC45gB3nuR6fY9GbBvRPMKQA salt-cd-infra@xvnet.eu.org

/etc/sudoers.d/salt-master-cd:
    file.managed:
        - makedirs: True
        - contents: |
            salt-master-cd ALL = (ALL:ALL) NOPASSWD:ALL