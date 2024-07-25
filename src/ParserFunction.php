<?php

namespace MediaWiki\Extension\Chart;

use MediaWiki\Html\Html;
use MediaWiki\Parser\Parser;
use MediaWiki\Shell\Shell;

class ParserFunction {
	/**
	 * Entry point for the `{{#chart:}}` parser function.
	 *
	 * @param Parser $parser
	 * @return array
	 */
	public function render( Parser $parser ) {
		if ( Shell::isDisabled() ) {
			// TODO i18n
			$error = Html::errorBox( 'Charts cannot be rendered because shell execution is disabled' );
			return [ $error, 'noparse' => true, 'isHTML' => true ];
		}
		$dir = dirname( __DIR__ ) . '/cli';
		$result = Shell::command(
			'node',
			'./dist/index.js',
			'line',
			// TODO provide input file (or stdin)
			'data.json',
			'-'
		)
			->workingDirectory( dirname( __DIR__ ) . '/cli' )
			->execute();
		// TODO check exit status for error
		$svg = $result->getStdout();
		// HACK work around a parser bug that inserts <p> tags even though we said not to parse
		$svg = str_replace( ">\n", '>', $svg );
		// Phan complains that we're outputting HTML that came from a shell command -- but we trust
		// our own shell script
		// @phan-suppress-next-line SecurityCheck-XSS
		return [ "<div class=\"mw-chart\">$svg</div>", 'noparse' => true, 'isHTML' => true ];
	}
}
