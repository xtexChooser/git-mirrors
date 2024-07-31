<?php

namespace MediaWiki\Extension\Chart;

use MediaWiki\Html\Html;
use MediaWiki\Parser\Parser;
use MediaWiki\Shell\Shell;

class ParserFunction {
	/**
	 * Renders an error
	 *
	 * @param string $errMsg
	 * @return array
	 */
	private function renderError( string $errMsg ) {
		$error = Html::errorBox( $errMsg );
		return [ $error, 'noparse' => true, 'isHTML' => true ];
	}

	/**
	 * Entry point for the `{{#chart:}}` parser function.
	 *
	 * @param Parser $parser
	 * @return array
	 */
	public function render( Parser $parser ) {
		if ( Shell::isDisabled() ) {
			return $this->renderError( $parser->msg( 'chart-error-shell-disabled' )->text() );
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
		$error = $result->getStderr();
		$svg = $result->getStdout();
		if ( $error ) {
			return $this->renderError(
				$parser->msg( 'chart-error-shell-error' )->text() .
				// TODO don't expose this in the HTML output, instead log to Logstash
				// and output a reqid that refers to it
				Html::element( 'blockquote', [ 'style' => 'display: none' ], $error )
			);
		}
		// If no SVG was returned, then the shell command didn't return anything, so treat this
		// as an error.
		if ( !trim( $svg ) ) {
			return $this->renderError( $parser->msg( 'chart-error-shell-no-response' )->text() );
		}
		// HACK work around a parser bug that inserts <p> tags even though we said not to parse
		$svg = str_replace( ">\n", '>', $svg );
		// Phan complains that we're outputting HTML that came from a shell command -- but we trust
		// our own shell script
		// @phan-suppress-next-line SecurityCheck-XSS
		return [ "<div class=\"mw-chart\">$svg</div>", 'noparse' => true, 'isHTML' => true ];
	}
}
