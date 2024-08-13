This is a MediaWiki extension for displaying charts.

For more information, see https://www.mediawiki.org/wiki/Extension:Chart

== Usage ==

Install, compile the service, and configure the Data: namespace per notes below.

Charts are expected to be configured on `.chart` pages in the Data: namespace, and
render using data from tabular `.tab` pages also in the Data: namespace.

Sample usage invoking both explicitly:

```
{{#chart:format=1993 Canadian federal election.chart
|data=1993 Canadian federal election.tab}}
```

== Installation ==
- Install the JsonConfig extension; see https://www.mediawiki.org/wiki/Extension:JsonConfig#Installation
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

== JsonConfig dependency ==

For the local `Data:` namespace, the `JsonConfig` extension is required to be running
and configured for "`.tab`" tabular data and "`.chart`" formatting definitions.

=== .tab tabular data ===

The "`.tab`" data pages are as per https://www.mediawiki.org/wiki/Help:Tabular_Data

Sample config for local development, with a central wiki sharing out its data pages:

```php
	// Safety: before extension.json, these values were initialized by JsonConfig.php
	$wgJsonConfigModels = $wgJsonConfigModels ?? [];
	$wgJsonConfigs = $wgJsonConfigs ?? [];

	// Tabular data is supported, but not configured on by default, in JsonConfig.
	// This should be simplified.
	// https://www.mediawiki.org/wiki/Extension:JsonConfig#Configuration
	$wgJsonConfigModels['Tabular.JsonConfig'] = 'JsonConfig\JCTabularContent';
	$wgJsonConfigs['Tabular.JsonConfig'] = [
		'namespace' => 486,
		'nsName' => 'Data',
		// page name must end in ".tab", and contain at least one symbol
		'pattern' => '/.\.tab$/',
		'license' => 'CC0-1.0',
		// allows the cache keys to be shared between wikis
		'isLocal' => false,
	];

	if ( $wgDBname === 'my_central_wiki' ) {
		$wgJsonConfigs['Tabular.JsonConfig']['store'] = true;
	} else {
		$wgJsonConfigs['Tabular.JsonConfig']['remote'] = [
			'url' => 'https://my-central-wiki.example.com/w/api.php'
		];
	}

```

A sample .tab page may be found in the `sample/` folder.

=== .chart format descriptions ===

The "`.chart`" data pages are custom for this extension and also build on `JsonConfig`.

```php
	// Safety: before extension.json, these values were initialized by JsonConfig.php
	$wgJsonConfigModels = $wgJsonConfigModels ?? [];
	$wgJsonConfigs = $wgJsonConfigs ?? [];

	// Chart data is supported here in the Chart extension, and currently must be
	// configured manually as well. This should be simplified.
	// https://www.mediawiki.org/wiki/Extension:JsonConfig#Configuration
	$wgJsonConfigModels['Chart.JsonConfig'] = 'MediaWiki\Extension\Chart\JCChartContent';
	$wgJsonConfigs['Chart.JsonConfig'] = [
		'namespace' => 486,
		'nsName' => 'Data',
		// page name must end in ".chart", and contain at least one symbol
		'pattern' => '/.\.chart$/',
		'license' => 'CC0-1.0',
		// allows the cache keys to be shared between wikis
		'isLocal' => false,
	];

	if ( $wgDBname === 'my_central_wiki' ) {
		$wgJsonConfigs['Chart.JsonConfig']['store'] = true;
	} else {
		$wgJsonConfigs['Chart.JsonConfig']['remote'] = [
			'url' => 'https://my-central-wiki.example.com/w/api.php'
		];
	}
```

Sample .chart page may be found under the `sample/` folder.
