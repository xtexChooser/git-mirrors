<?php

namespace MediaWiki\Extension\Chart;

use MediaWiki\Logger\LoggerFactory;
use MediaWiki\Shell\Shell;
use MediaWiki\Status\Status;
use MWCryptHash;
use stdclass;

class ChartRenderer {
	/**
	 * Render a chart from a definition object and a tabular data object.
	 *
	 * @param stdclass $chartDef Chart definition, obtained from JCChartContent::getContent()
	 * @param stdclass $tabularData Tabular data, obtained from JCTabularContent::getContent()
	 * @param array{width?:string,height?:string} $options Additional rendering options:
	 *   'width': Width of the chart, in pixels. Overrides width specified in the chart definition
	 *   'height': Height of the chart, in pixels. Overrides height specified in the chart definition.
	 * @return Status A Status object wrapping an SVG string or an error
	 */
	public function renderSVG( stdclass $chartDef, stdclass $tabularData, array $options = [] ): Status {
		// Prefix for IDs in the SVG. This has to be unique between charts on the same page, to
		// prevent ID collisions (T371558). If the same chart with the same data is displayed twice
		// on the same page, this gives them the same ID prefixes and causes their IDs to collide,
		// but that doesn't seem to cause a problem in practice.
		$definitionForHash = json_encode( [ 'format' => $chartDef, 'source' => $tabularData ] );
		$chartDef = clone $chartDef;
		$chartDef->idPrefix = 'mw-chart-' . MWCryptHash::hash( $definitionForHash, false );

		$chartPath = tempnam( \wfTempDir(), 'chart-json' );
		file_put_contents( $chartPath, json_encode( $chartDef ) );
		$dataPath = tempnam( \wfTempDir(), 'data-json' );
		file_put_contents( $dataPath, json_encode( $tabularData ) );

		$shellArgs = $this->getShellArgs( $dataPath, $chartPath, $options );

		$result = Shell::command( ...$shellArgs )
			->workingDirectory( dirname( __DIR__ ) . '/cli' )
			->execute();

		$error = $result->getStderr();
		if ( $error ) {
			// TODO use dependency injection for LoggerFactory
			LoggerFactory::getInstance( 'Chart' )->warning(
				'Chart shell command returned error: {error}',
				[ 'error' => $error ]
			);

			// @todo tracking category
			$status = Status::newFatal( 'chart-error-shell-error' );
		} else {
			$svg = $result->getStdout();
			$status = Status::newGood( $svg );
		}

		unlink( $chartPath );
		unlink( $dataPath );
		return $status;
	}

	/**
	 * @param string $dataPath
	 * @param string $chartPath
	 * @param array{width?:string,height?:string} $options
	 * @return string[]
	 */
	private function getShellArgs( string $dataPath, string $chartPath, array $options ): array {
		$shellArgs = [
			'node',
			'./dist/index.js',
			// TODO support more chart types
			'line',
			$dataPath,
			$chartPath,
			'-'
		];

		if ( isset( $options['width'] ) ) {
			$shellArgs[] = "--width";
			$shellArgs[] = $options['width'];
		}

		if ( isset( $options['height'] ) ) {
			$shellArgs[] = "--height";
			$shellArgs[] = $options['height'];
		}

		return $shellArgs;
	}
}
