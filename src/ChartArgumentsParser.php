<?php

namespace MediaWiki\Extension\Chart;

use MediaWiki\Parser\Parser;

class ChartArgumentsParser {
	private DataPageResolver $dataPageResolver;

	public function __construct( DataPageResolver $dataPageResolver ) {
		$this->dataPageResolver = $dataPageResolver;
	}

	public function parseArguments( Parser $parser, array $args ): ParsedArguments {
		$magicWords = $parser->getMagicWordFactory()->newArray( [
			'chart_data',
			'chart_width',
			'chart_height'
		] );

		$definition = array_shift( $args );
		$dataSource = null;
		$options = [];
		$errors = [];
		foreach ( $args as $arg ) {
			if ( str_contains( $arg, '=' ) ) {
				[ $key, $value ] = array_map( 'trim', explode( '=', $arg, 2 ) );
				switch ( $magicWords->matchStartToEnd( $key ) ) {
					case 'chart_data':
						$dataSource = $value;
						break;
					// @unstable: @todo revisit after T371712
					case 'chart_width':
						$filteredValue = filter_var( $value, FILTER_VALIDATE_INT, [
							'options' => [ 'min_range' => 100 ]
						] );

						if ( $filteredValue === false ) {
							$errors[] = [
								'key' => 'chart-error-invalid-width',
								'params' => [ $value ]
							];
						} else {
							$options[ 'width' ] = $filteredValue;
						}
						break;
					// @unstable: @todo revisit after T371712
					case 'chart_height':
						$filteredValue = filter_var( $value, FILTER_VALIDATE_INT, [
							'options' => [ 'min_range' => 100 ]
						] );

						if ( $filteredValue === false ) {
							$errors[] = [
								'key' => 'chart-error-invalid-height',
								'params' => [ $value ]
							];
						} else {
							$options[ 'height' ] = $filteredValue;
						}
						break;
					default:
						// no-op
				}
			}
		}

		if ( !$definition ) {
			$errors[] = [
				'key' => 'chart-error-chart-definition-not-found',
				'params' => []
			];
		}

		$definitionTitle = $this->dataPageResolver->resolvePageInDataNamespace( $definition );
		if ( !$definitionTitle ) {
			$errors[] = [
				'key' => 'chart-error-chart-definition-not-found',
				'params' => []
			];
		}

		$dataTitle = null;
		if ( $dataSource !== null ) {
			$dataTitle = $this->dataPageResolver->resolvePageInDataNamespace( $dataSource );
			if ( !$dataTitle ) {
				$errors = [
					'key' => 'chart-error-chart-definition-not-found',
					'params' => []
				];
			}
		}

		return new ParsedArguments( $definitionTitle, $dataTitle, $options, $errors );
	}

}
