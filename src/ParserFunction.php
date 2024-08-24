<?php

namespace MediaWiki\Extension\Chart;

use JsonConfig\JCSingleton;
use JsonConfig\JCTabularContent;
use Language;
use MediaWiki\Html\Html;
use MediaWiki\MediaWikiServices;
use MediaWiki\Message\Message;
use MediaWiki\Page\PageReference;
use MediaWiki\Parser\Parser;
use MediaWiki\Parser\ParserOutput;
use MediaWiki\Title\Title;
use MessageLocalizer;
use WikiPage;

class ParserFunction implements MessageLocalizer {

	private Language $language;

	/** @var ?PageReference */
	private $page;

	private ChartRenderer $chartRenderer;

	public function __construct( ChartRenderer $chartRenderer, Language $language, ?PageReference $page ) {
		$this->chartRenderer = $chartRenderer;
		$this->language = $language;
		$this->page = $page;
	}

	/**
	 * @inheritDoc
	 */
	public function msg( $key, ...$params ): Message {
		return wfMessage( $key, ...$params )->inLanguage( $this->language )->page( $this->page );
	}

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
	 * Static entry point for the `{{#chart:}}` parser function.
	 *
	 * Wrapper for render() that creates an instance of this class based on the Parser's state.
	 * This is needed because these Parser getters don't work yet in the ParserFirstCallInit hook.
	 *
	 * @param Parser $parser
	 * @param string ...$args
	 * @return array
	 */
	public static function funcHook( Parser $parser, ...$args ) {
		$chartRenderer = MediaWikiServices::getInstance()->getService( 'Chart.ChartRenderer' );
		$instance = new static( $chartRenderer, $parser->getTargetLanguage(), $parser->getPage() );
		return $instance->render( $parser, ...$args );
	}

	/**
	 * Main entry point for the `{{#chart:}}` parser function.
	 *
	 * @param Parser $parser
	 * @param string ...$args
	 * @return array
	 */
	public function render( Parser $parser, ...$args ) {
		// @todo incrementExpensiveFunctionCount

		$magicWords = $parser->getMagicWordFactory()->newArray( [
			'chart_data',
			'chart_width',
			'chart_height'
		] );

		$definition = array_shift( $args );
		$dataSource = null;
		$options = [];
		foreach ( $args as $arg ) {
			if ( str_contains( $arg, '=' ) ) {
				[ $key, $value ] = array_map( 'trim', explode( '=', $arg, 2 ) );
				switch ( $magicWords->matchStartToEnd( $key ) ) {
					case 'chart_data':
						$dataSource = $value;
						break;
					// @unstable: @todo revisit after T371712
					case 'chart_width':
						$options['width'] = $value;
						break;
					// @unstable: @todo revisit after T371712
					case 'chart_height':
						$options['height'] = $value;
						break;
					default:
						// no-op
				}
			}
		}

		if ( !$definition ) {
			return $this->renderError( $this->msg( 'chart-error-chart-definition-not-found' )->text() );
		}
		$definitionTitle = $this->resolvePageInDataNamespace( $definition );
		if ( !$definitionTitle ) {
			return $this->renderError( $this->msg( 'chart-error-chart-definition-not-found' )->text() );
		}
		$dataTitle = null;
		if ( $dataSource !== null ) {
			$dataTitle = $this->resolvePageInDataNamespace( $dataSource );
			if ( !$dataTitle ) {
				return $this->renderError( $this->msg( 'chart-error-chart-definition-not-found' )->text() );
			}
		}

		$html = $this->renderChart( $parser->getOutput(), $definitionTitle, $dataTitle, $options );

		// HACK work around a parser bug that inserts <p> tags even though we said not to parse
		$html = str_replace( ">\n", '>', $html );
		return [ $html, 'noparse' => true, 'isHTML' => true ];
	}

	/**
	 * Look up a page in the Data: namespace. This takes a string like "Foo.tab" and returns a
	 * Title object corresponding to Data:Foo.tab.
	 *
	 * @param string $pageName Name of a Data page, without the namespace prefix
	 * @return ?Title Title object for that page in the Data: namespace (or null if invalid)
	 */
	private function resolvePageInDataNamespace( string $pageName ): ?Title {
		// TODO we should provide this setting and the namespace ourselves, so that we don't have
		// to rely on the admin to set it up in the config
		$config = MediaWikiServices::getInstance()->getMainConfig();
		$dataNs = $config->get( 'JsonConfigs' )['Chart.JsonConfig']['namespace'];
		return Title::newFromText( $pageName, $dataNs );
	}

	/**
	 * Render a chart from a definition page and a tabular data page.
	 *
	 * @param ParserOutput $output ParserOutput the chart is being rendered into. Used to record
	 *   dependencies on the chart and data pages.
	 * @param Title|JCChartContent $chartDefinition Chart definition page. If this is a
	 *   JCChartContent object, that content will be used directly; if it's a PageIdentity, the
	 *   content of that page will be fetched.
	 * @param ?Title $tabularData Tabular data page. If this is not set, the default source
	 *   page specified on the chart definition page will be used.
	 * @param array{width?:string,height?:string} $options Additional rendering options:
	 *   'width': Width of the chart, in pixels. Overrides width specified in the chart definition
	 *   'height': Height of the chart, in pixels. Overrides height specified in the chart definition.
	 * @return string HTML
	 */
	public function renderChart(
		ParserOutput $output,
		$chartDefinition,
		?Title $tabularData = null,
		array $options = []
	): string {
		if ( $chartDefinition instanceof JCChartContent ) {
			$definitionContent = $chartDefinition;
		} else {
			$definitionPage = new WikiPage( $chartDefinition );
			$definitionContent = JCSingleton::getContent( $chartDefinition->getTitleValue() );
			if ( !$definitionContent ) {
				return Html::errorBox( $this->msg( 'chart-error-chart-definition-not-found' )->text() );
			}
			if ( !( $definitionContent instanceof JCChartContent ) ) {
				return Html::errorBox( $this->msg( 'chart-error-incompatible-chart-definition' )->text() );
			}
			if ( $definitionPage->exists() ) {
				// Record a dependency on the chart page, so that the page embedding the chart
				// is reparsed when the chart page is edited
				$output->addTemplate(
					$definitionPage->getTitle(),
					$definitionPage->getId(),
					$definitionPage->getRevisionRecord()->getId()
				);
			} else {
				// @todo register cross-site dependencies using extended GlobalUsage
				// and allow updates to trigger re-parses of affected pages
			}
		}
		$definitionObj = $definitionContent->getLocalizedData( $this->language );

		if ( !$tabularData ) {
			if ( !isset( $definitionObj->source ) ) {
				return Html::errorBox( $this->msg( 'chart-error-default-source-not-specified' )->text() );
			}
			$tabularData = $this->resolvePageInDataNamespace( $definitionObj->source );
			if ( !$tabularData ) {
				return Html::errorBox( $this->msg( 'chart-error-data-source-page-not-found' )->text() );
			}
		}
		$dataPage = new WikiPage( $tabularData );
		$dataContent = JCSingleton::getContent( $tabularData->getTitleValue() );
		if ( !$dataContent ) {
			return Html::errorBox( $this->msg( 'chart-error-data-source-page-not-found' )->text() );
		}
		if ( !( $dataContent instanceof JCTabularContent ) ) {
			return Html::errorBox( $this->msg( 'chart-error-incompatible-data-source' )->text() );
		}
		if ( $dataPage->exists() ) {
			// Record a dependency on the data page, so that the page embedding the chart
			// is reparsed when the data page is edited
			$output->addTemplate(
				$dataPage->getTitle(),
				$dataPage->getId(),
				$dataPage->getRevisionRecord()->getId()
			);
		} else {
			// @todo register cross-site dependencies using extended GlobalUsage
			// and allow updates to trigger re-parses of affected pages
		}
		$dataObj = $dataContent->getLocalizedData( $this->language );

		$status = $this->chartRenderer->renderSVG( $definitionObj, $dataObj, $options );
		if ( !$status->isGood() ) {
			return Html::errorBox( $status->getHTML() );
		}
		$svg = $status->getValue();
		return Html::rawElement( 'div', [ 'class' => 'mw-chart' ], $svg );
	}
}
