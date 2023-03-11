# DN42 Kioubit Canvas WP RC Renderer

This tool is used to draw the latest Wikipedia recent changes on the [Kioubit DN42 Canvas](http://us2.g-load.eu:9090/).

[crates.io](https://crates.io/crates/dn42-kb-canvas-wikipedia-rc)

[codeberg.org OCI image](https://codeberg.org/xtex/-/packages/container/dn42-kb-canvas-wikipedia-rc/latest)

[codeberg.org](https://codeberg.org/xtex/dn42-kb-canvas-wikipedia-rc)

Currently deployed on XTEX-VNET (AS4242420361) (2023/03/11).

Running: `podman run --cap-add CAP_NET_RAW --name dn42-kb-canvas-wikipedia-rc -d --network host codeberg.org/xtex/dn42-kb-canvas-wikipedia-rc`

ICMPv6 ECHO Request will be sent with identifier `4431`.
