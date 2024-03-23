<?php

namespace MediaWiki\Extension\GloopTweaks\ResourceLoader;

use MediaWiki\MediaWikiServices;
use MediaWiki\Revision\SlotRecord;
use ResourceLoaderContext;
use ResourceLoaderSiteStylesModule;

class FixedWidthStyleModule extends ResourceLoaderSiteStylesModule {
	/**
	 * @param string $titleText
	 * @param ResourceLoaderContext $context
	 * @return null|string
	 * @since 1.32 added the $context parameter
	 */
	protected function getContent( $titleText, ResourceLoaderContext $context ) {
		global $wgGloopTweaksCentralDB;
		$services = MediaWikiServices::getInstance();

		$title = $services->getTitleParser()->parseTitle( $titleText );
		$store = $services->getRevisionStoreFactory()->getRevisionStore( $wgGloopTweaksCentralDB );
		$rev = $store->getRevisionByTitle( $title );

		$content = $rev ? $rev->getContent( SlotRecord::MAIN ) : null;
		if ( !$content ) {
			return null; // No content found
		}

		$handler = $content->getContentHandler();
		if ( $handler->isSupportedFormat( CONTENT_FORMAT_CSS ) ) {
			$format = CONTENT_FORMAT_CSS;
		} elseif ( $handler->isSupportedFormat( CONTENT_FORMAT_JAVASCRIPT ) ) {
			$format = CONTENT_FORMAT_JAVASCRIPT;
		} else {
			return null; // Bad content model
		}

		return $content->serialize( $format );
	}

	// Override getDB() to use metawiki rather than having a per-wiki MediaWiki:Vector-fixedwidth.css.
	protected function getDB() {
		global $wgGloopTweaksCentralDB;
		$lbFactory = MediaWikiServices::getInstance()->getDBLoadBalancerFactory();
		$lb = $lbFactory->getMainLB( $wgGloopTweaksCentralDB );
		return $lb->getConnectionRef( DB_REPLICA, [], $wgGloopTweaksCentralDB );
	}

	/**
	 * Get list of pages used by this module
	 *
	 * @param ResourceLoaderContext $context
	 * @return array[]
	 */
	protected function getPages( ResourceLoaderContext $context ) {
		$pages = [];
		if ( $this->getConfig()->get( 'UseSiteCss' ) ) {
			$skin = $context->getSkin();
			$pages['MediaWiki:' . ucfirst( $skin ) . '-fixedWidth.css'] = [ 'type' => 'style' ];
		}
		return $pages;
	}

	// 'site' should be used, but can't as this module needs to load after 'site.styles'.
	public function getGroup() {
		return 'user';
	}
}
