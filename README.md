This is a MediaWiki extension for displaying charts.

For more information, see https://www.mediawiki.org/wiki/Extension:Chart

== Installation ==
- Clone this repo into the `extensions/Chart` directory in your MediaWiki installation
- In the `extensions/Chart` directory, run `npm install && npm run -w cli build`
- Add `wfLoadExtension( 'Chart' )` at the bottom of `LocalSettings.php`

== Additional setup steps for MediaWiki-Docker ==
If you're using MediaWiki-Docker, add the following to the `docker-compose.override.yml` file in the
MediaWiki directory:
```
services:
  mediawiki:
    build:
      context: ./extensions/Chart
      dockerfile: nodejs.dockerfile
```

Then run:
```
docker compose build
docker compose down
docker compose up -d
```
