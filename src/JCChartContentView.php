<?php

namespace MediaWiki\Extension\Chart;

use JsonConfig\JCContent;
use JsonConfig\JCContentView;
use MediaWiki\MediaWikiServices;
use MediaWiki\Page\PageReference;
use MediaWiki\Parser\ParserOutput;
use MediaWiki\Title\Title;
use ParserOptions;

class JCChartContentView extends JCContentView {
	/**
	 * @param JCContent $content
	 * @param PageReference $page
	 * @param int|null $revId
	 * @param ParserOptions $options
	 * @param bool $generateHtml
	 * @param ParserOutput &$output
	 * @return string
	 */
	public function valueToHtml(
		JCContent $content, PageReference $page, $revId, ParserOptions $options, $generateHtml,
		ParserOutput &$output
	): string {
		'@phan-var JCChartContent $content';
		// TODO use dependency injection?
		$languageFactory = MediaWikiServices::getInstance()->getLanguageFactory();
		$lang = $languageFactory->getLanguage( $output->getLanguage() ??
			Title::newFromPageReference( $page )->getPageLanguage()
		);
		$parserFunction = new ParserFunction( $lang, $page );

		return $parserFunction->renderChart( $output, $content );
	}

	/**
	 * @inheritDoc
	 */
	public function getDefault( $modelId ): string {
		$licenseIntro = JCContentView::getLicenseIntro();
		return <<<JSON
{
	// !!!!! All comments will be automatically deleted on save !!!!!
	"version": 1,

	$licenseIntro
	
	// Default width and height of the chart. Can be overridden on each page that uses the chart.
	"width": 600,
	"height": 400,

	// Name of a tabular data page to use as the data source. Can be overridden on each page that uses the chart.
	"source": "",

	// Chart type. Available types are: line
	"type": "line",

	// Axis labels and other axis settings
	"xAxis": {
		"title": {
			"en": "X axis label"
		}
	},
	"yAxis": {
		"title": {
			"en": "Y axis label"
		}
	}

	// Other chart parameters, these are specific to each chart type
}
JSON;
	}
}
