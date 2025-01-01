---
name: '[MW] Create a new wiki'
about: 'Request to create a new wiki.'
title: 'MW: Create wiki <NAME>'
labels:
  - mw
  - new
---

Per previous discussion, creation of wiki (NAME) is accepted.

## Site Configuration

TODO: Needs details

## Tasks

- [ ] (opt) Pre-creation confirmation with the community.
- [ ] (opt) Add new network operators.
- [ ] (opt) Add new extensions and skins.
- [ ] Add a issue label on Codeberg.
- [ ] Add site configuration code to `services/mediawiki/config/sites/`.
- [ ] Add site to `services/mediawiki/config/sites.json`.
- [ ] Add Caddy configuration to `services/mediawiki/Caddyfile`.
- [ ] Run `atre s mediawiki addwiki <WIKI>` to create wiki.
- [ ] (opt) Run `atre s mediawiki addcargo <WIKI>` to create Cargo database.
- [ ] (opt) Create operators accounts and grant staff groups.
- [ ] Add meta-wiki interwiki.
- [ ] Grant bureaucrat groups.
- [ ] (opt) Coordinate DNS records for custom domains.
- [ ] Run `atre s mediawiki cronjob <WIKI>` to generate initial sitemaps.
- [ ] (opt) Submit sitemaps to Google and Bing.
- [ ] Add to uptime checker.
