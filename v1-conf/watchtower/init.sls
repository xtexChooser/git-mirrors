watchtower:
    docker_image.present:
        - name: docker.io/containrrr/watchtower
        - tag: latest
        - require:
            - test: container
    docker_container.running:
        - image: docker.io/containrrr/watchtower:latest
        - binds:
            - /var/run/docker.sock:/var/run/docker.sock
            - /etc/localtime:/etc/localtime:ro
        - require:
            - test: container
            - docker_image: watchtower
        - hostname: watchtower
        - environment:
            - HOME: /root
            - WATCHTOWER_NOTIFICATIONS_HOSTNAME: {{ pillar['name'] }}
            - WATCHTOWER_NOTIFICATION_URL: ntfy://:{{ salt['pillar.fetch']('watchtower:ntfy-token') }}@ntfy.xvnet.eu.org/watchtower?Title={{ pillar['name'] }}
            - WATCHTOWER_CLEANUP: true
#            - WATCHTOWER_INCLUDE_RESTARTING: true
            - WATCHTOWER_INCLUDE_STOPPED: true
            {# 15 mins -#}
            - WATCHTOWER_POLL_INTERVAL: 900
        - memory: 32M
