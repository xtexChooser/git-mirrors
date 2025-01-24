<?php

namespace MediaWiki\Extension\Chart;

use MediaWiki\MediaWikiServices;
use MediaWiki\Title\Title;

class DataPageResolver {
	/**
	 * Look up a page in the Data: namespace. This takes a string like "Foo.tab" and returns a
	 * Title object corresponding to Data:Foo.tab.
	 *
	 * @param string $pageName Name of a Data page, without the namespace prefix
	 * @return ?Title Title object for that page in the Data: namespace (or null if invalid)
	 */
	public function resolvePageInDataNamespace( string $pageName ): ?Title {
		// TODO we should provide this setting and the namespace ourselves, so that we don't have
		// to rely on the admin to set it up in the config
		$config = MediaWikiServices::getInstance()->getMainConfig();
		$dataNs = $config->get( 'JsonConfigs' )['Chart.JsonConfig']['namespace'];
		return Title::newFromText( $pageName, $dataNs );
	}

}
