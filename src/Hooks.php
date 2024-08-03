<?php

namespace MediaWiki\Extension\Chart;

use MediaWiki\Hook\ParserFirstCallInitHook;
use MediaWiki\Parser\Parser;

class Hooks implements ParserFirstCallInitHook {
	/**
	 * @param Parser $parser
	 */
	public function onParserFirstCallInit( $parser ) {
		$parser->setFunctionHook( 'chart', [ ParserFunction::class, 'funcHook' ] );
	}
}
