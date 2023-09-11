# OpenLDAP OCI Image

This is an OCI-compatible container image for OpenLDAP.

## Build

Current build matrix can be found at [.woodpecker.yml#L4](https://codeberg.org/xvnet/openldap-oci/src/branch/main/.woodpecker.yml#L4).

[CI](https://ci.codeberg.org/xvnet/openldap-oci) is provided by Codeberg CI.

## Usage

Image is hosted by Codeberg. `codeberg.org/xvnet/openldap:VERSION`.

By default, `slapd` is the default target (which is wrapped with start script `/olo/start.sh`).

Default listening addresses are `ldap:/// ldaps:/// ldapi:///`.

Max open FDs are limited to 1024 (`ulimit -n 1024`) in order to reduce memory usage.

`/etc/openldap/slapd.ldif` will be added through `slapadd` on launch.

Configuration file should be mounted at `/etc/openldap/slapd.conf`.

Default `slapd.conf` and `slapd.ldif` will be uploaded to `sprunge.us` by builder, and can be found in the build log. For example, http://sprunge.us/XVIHxQ and http://sprunge.us/Ccj4Cb.

Builtin-schemas are available under `/olo/schema`.

## License

The OpenLDAP project is licensed under The OpenLDAP Public License. A copy is at `OpenLDAP-LICENSE`.

This container image is licensed under Apache-2.0 License. A copy is at `LICENSE`.

