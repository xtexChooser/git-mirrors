#echo:
#    docker_image.absent:
#        - images:
#            - k8s.gcr.io/echoserver
#        - force: True
#    docker_container.absent:
#        - force: True

/home/xtex/foo.conf:
    file.absent

#bird:
#  file.managed:
#    - source: salt://apache/http.conf
#    - user: root
#    - group: root
#    - mode: 644
#    - attrs: ai
#    - template: jinja
#    - defaults:
#        custom_var: "default value"
#        other_var: 123
#{% if grains['os'] == 'Ubuntu' %}
#    - context:
#        custom_var: "override"
#{% endif %}
