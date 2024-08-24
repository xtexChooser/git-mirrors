<?php

namespace MediaWiki\Extension\Chart;

use MediaWiki\Config\ServiceOptions;
use MediaWiki\Logger\LoggerFactory;
use MediaWiki\MediaWikiServices;

/**
 * @codeCoverageIgnore
 */

/** @phpcs-require-sorted-array */
return [
	'Chart.ChartRenderer' => static function ( MediaWikiServices $services ): ChartRenderer {
		return new ChartRenderer(
			new ServiceOptions(
				ChartRenderer::CONSTRUCTOR_OPTIONS,
				$services->getMainConfig()
			),
			$services->getHttpRequestFactory(),
			$services->getFormatterFactory(),
			LoggerFactory::getInstance( 'Chart' )
		);
	},
];
