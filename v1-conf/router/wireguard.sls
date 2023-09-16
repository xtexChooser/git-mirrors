wireguard:
    pkg.installed:
        - pkgs:
{% if grains.os_family != 'Arch' %}
            - wireguard
{%- endif %}
            - wireguard-tools
