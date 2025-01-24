<?php

namespace MediaWiki\Extension\Chart;

use MediaWiki\Title\Title;

class ParsedArguments {
	private ?Title $definitionPageTitle;

	private ?Title $dataPageTitle;

	private array $options;

	/**
	 * @var array[] List of errors, with 'key' and 'params'
	 */
	private array $errors;

	/**
	 * @param ?Title $definitionPageTitle Chart definition page title.
	 * @param ?Title $dataPageTitle Tabular data page.
	 * @param array{width?:string,height?:string} $options Additional rendering options:
	 *    'width': Width of the chart, in pixels. Overrides width specified in the chart definition
	 *    'height': Height of the chart, in pixels. Overrides height specified in the chart definition.
	 * @param array<array{key:string, params:array}> $errors An array of errors with key and params
	 */
	public function __construct(
		?Title $definitionPageTitle,
		?Title $dataPageTitle,
		array $options,
		array $errors
	) {
		$this->definitionPageTitle = $definitionPageTitle;
		$this->dataPageTitle = $dataPageTitle;
		$this->options = $options;
		$this->errors = $errors;
	}

	public function getDefinitionPageTitle(): ?Title {
		return $this->definitionPageTitle;
	}

	public function getDataPageTitle(): ?Title {
		return $this->dataPageTitle;
	}

	public function getOptions(): array {
		return $this->options;
	}

	public function getErrors(): array {
		return $this->errors;
	}
}
