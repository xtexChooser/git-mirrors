<?php

namespace MediaWiki\Extension\Chart;

use MediaWiki\Logger\LoggerFactory;
use MediaWiki\MediaWikiServices;

/**
 * @codeCoverageIgnore
 */

/** @phpcs-require-sorted-array */
return [
	'Chart.ChartRenderer' => static function ( MediaWikiServices $services ): ChartRenderer {
		return new ChartRenderer(
			LoggerFactory::getInstance( 'Chart' )
		);
	},
];
