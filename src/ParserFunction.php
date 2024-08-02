<?php

namespace MediaWiki\Extension\Chart;

use JsonConfig\JCTabularContent;
use MediaWiki\Html\Html;
use MediaWiki\Logger\LoggerFactory;
use MediaWiki\MediaWikiServices;
use MediaWiki\Parser\Parser;
use MediaWiki\Shell\Shell;
use MediaWiki\Status\Status;
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
		$logger = LoggerFactory::getInstance( 'Chart' );

		// @todo incrementExpensiveFunctionCount

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

				// Record a dependency on the chart page, so that the page embedding the chart
				// is reparsed when the chart page is edited
				$parser->getOutput()->addTemplate(
					$formatPage->getTitle(),
					$formatPage->getId(),
					$formatPage->getRevisionRecord()->getId()
				);
			}
		} else {
			return $this->renderError( $parser->msg( 'chart-error-chart-definition-not-found' )->text() );
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

				// Record a dependency on the data page, so that the page embedding the chart
				// is reparsed when the data page is edited
				$parser->getOutput()->addTemplate(
					$sourcePage->getTitle(),
					$sourcePage->getId(),
					$sourcePage->getRevisionRecord()->getId()
				);
			} else {
				return $this->renderError( $parser->msg( 'chart-error-incompatible-data-source' )->text() );
			}
		} else {
			return $this->renderError( $parser->msg( 'chart-error-data-source-page-not-found' )->text() );
		}

		$chartPath = tempnam( \wfTempDir(), 'chart-json' );
		file_put_contents( $chartPath, json_encode( $formatData ) );
		$sourcePath = tempnam( \wfTempDir(), 'data-json' );
		file_put_contents( $sourcePath, json_encode( $sourceData ) );

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

		$error = $result->getStderr();
		if ( $error ) {
			$logger->warning( 'Chart shell command returned error: {error}', [
				'error' => $error
			] );

			// @todo tracking category
			$status = Status::newFatal( 'chart-error-shell-error' );
		} else {
			$svg = $result->getStdout();

			// HACK work around a parser bug that inserts <p> tags even though we said not to parse
			$svg = str_replace( ">\n", '>', $svg );

			// Phan complains that we're outputting HTML that came from a shell command -- but we trust
			// our own shell script
			$chartOutput = [ "<div class=\"mw-chart\">$svg</div>", 'noparse' => true, 'isHTML' => true ];
			$status = Status::newGood( $chartOutput );
		}

		unlink( $chartPath );
		unlink( $sourcePath );

		if ( !$status->isGood() ) {
			// If no SVG was returned, then the Shell command didn't return anything so treat this an error
			return $this->renderError( $parser->msg( 'chart-error-shell-error' )->text() );
		}

		return $status->getValue();
	}
}
