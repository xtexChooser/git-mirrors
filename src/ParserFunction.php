<?php

namespace MediaWiki\Extension\Chart;

use JsonConfig\JCTabularContent;
use MediaWiki\Html\Html;
use MediaWiki\MediaWikiServices;
use MediaWiki\Parser\Parser;
use MediaWiki\Shell\Shell;
use MediaWiki\Title\Title;
use WikiPage;

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
	 * @param string ...$args
	 * @return array
	 */
	public function render( Parser $parser, ...$args ) {
		if ( Shell::isDisabled() ) {
			return $this->renderError( $parser->msg( 'chart-error-shell-disabled' )->text() );
		}

		$format = null;
		$data = null;
		foreach ( $args as $arg ) {
			// @fixme use proper i18n-friendly magic words
			if ( strpos( $arg, '=' ) >= 0 ) {
				[ $key, $value ] = explode( '=', $arg, 2 );
				switch ( $key ) {
					case 'format':
						$format = $value;
						break;
					case 'data':
						$data = $value;
						break;
					default:
						// no-op
				}
			}
		}

		// TODO use dependency injection
		$config = MediaWikiServices::getInstance()->getMainConfig();
		$ns = $config->get( 'JsonConfigs' )['Chart.JsonConfig']['namespace'];

		$formatData = (object)[];
		$formatTitle = Title::newFromText( $format, $ns );
		if ( $formatTitle ) {
			$formatPage = new WikiPage( $formatTitle );
			$formatContent = $formatPage->getContent();
			if ( $formatContent instanceof JCChartContent ) {
				// @fixme remote will require going through JCSingleton::getContent?
				//$formatContent = JCSingleton::getContent( $format->getTitleValue() );
				$formatData = $formatContent->getLocalizedData( $parser->getTargetLanguage() );
			}
		} else {
			var_dump( 'no format' );
		}

		$sourceData = (object)[ 'fields' => [], 'data' => [] ];
		$sourceTitle = Title::newFromText( $data, $ns );
		if ( $sourceTitle ) {
			$sourcePage = new WikiPage( $sourceTitle );
			$sourceContent = $sourcePage->getContent();
			if ( $sourceContent instanceof JCTabularContent ) {
				// @fixme remote will require going through JCSingleton::getContent?
				//$sourceContent = JCSingleton::getContent( $source->getTitleValue() );
				$sourceData = $sourceContent->getLocalizedData( $parser->getTargetLanguage() );
			} else {
				var_dump( 'incomptable data source' );
			}
		} else {
			var_dump( 'no data' );
		}

		$chartPath = tempnam( \wfTempDir(), 'chart-json' );
		file_put_contents( $chartPath, json_encode( $formatData ) );
		$sourcePath = tempnam( \wfTempDir(), 'data-json' );
		file_put_contents( $sourcePath, json_encode( $sourceData ) );

		$dir = dirname( __DIR__ ) . '/cli';
		$result = Shell::command(
			'node',
			'./dist/index.js',
			'line',
			$sourcePath,
			$chartPath,
			'-'
		)
			->workingDirectory( dirname( __DIR__ ) . '/cli' )
			->execute();
		unlink( $chartPath );
		unlink( $sourcePath );
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
