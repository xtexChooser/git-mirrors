<?php

namespace MediaWiki\Extension\Chart;

use MediaWiki\Hook\ParserFirstCallInitHook;

class Hooks implements ParserFirstCallInitHook {
	/** @var ParserFunction */
	private $mParserFunction;

	public function __construct() {
		$this->mParserFunction = new ParserFunction();
	}

	/**
	 * @param Parser $parser
	 */
	public function onParserFirstCallInit( $parser ) {
		$parser->setFunctionHook( 'chart', [ $this->mParserFunction, 'render' ] );
	}
}
